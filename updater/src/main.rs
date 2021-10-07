use std::{ffi::OsStr, fs::File, io::{Read, Write}, path::{PathBuf, Path}, collections::{BTreeMap, hash_map::DefaultHasher}, hash::{Hash, Hasher}};

use color_eyre::eyre::{eyre, Result};
use dunce::canonicalize;
use git2::Repository;
use heck::{CamelCase, KebabCase, SnakeCase};
use itertools::Itertools;
use semver::Version;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use toml::Value;
use walkdir::WalkDir;

#[derive(Clone, Debug, StructOpt)]
struct Args {
	/// Directory to output the crate in
	#[structopt(long, value_name = "dir path")]
	output: PathBuf,

	/// File containing the base Cargo manifest
	#[structopt(long, value_name = "file path")]
	cargo_toml_base: PathBuf,

	/// Repository (local path) containing a checkout of https://github.com/github/gitignore
	#[structopt(long, value_name = "dir path")]
	gitignores_repo: PathBuf,

	/// When running in GitHub Actions, this should be "$GITHUB_ENV"
	#[structopt(long, value_name = "file path")]
	gh_env_file: Option<PathBuf>,
}

fn main() -> Result<()> {
	color_eyre::install()?;
	let mut args = Args::from_args();

	args.output = canonicalize(args.output)?;
	args.cargo_toml_base = canonicalize(args.cargo_toml_base)?;
	args.gitignores_repo = canonicalize(args.gitignores_repo)?;
	if let Some(gh) = args.gh_env_file.take() {
		args.gh_env_file = Some(canonicalize(gh)?);
	}

	let repo = Repository::open(&args.gitignores_repo)?;
	let commit = repo.head()?.peel_to_commit()?.id().to_string();

	let mut entries = Vec::with_capacity(512);
	for path in WalkDir::new(&args.gitignores_repo).follow_links(true).into_iter().filter_map(|e| e.ok().and_then(|e| {
		match e.path() {
			path if path.extension() == Some(OsStr::new("gitignore")) => Some(path.to_owned()),
			_ => None,
		}
	})) {
		entries.push(GitIgnore::new(&path, &args.gitignores_repo, &commit)?);
	}

	let mut default_features = vec!["std".to_string()];
	let mut features = BTreeMap::new();
	features.insert("no-contents".to_string(), Vec::new());
	features.insert("std".to_string(), Vec::new());

	let mut content_hash = DefaultHasher::new();

	entries.sort_by_key(|e| e.feature.clone());
	for (name, variants) in &entries.into_iter().group_by(|e| e.collection.clone()) {
		let collection = Collection::new(&name, variants);
		let mut module = File::create(&args.output.join("src").join(name.to_snake_case()).with_extension("rs"))?;
		module.write_all(collection.generate_module().as_bytes())?;
		features.extend(collection.feature_list());
		default_features.push(collection.feature.clone());
		collection.hash(&mut content_hash);
	}

	let content_hash = content_hash.finish();
	features.insert("default".to_string(), default_features);

	let cargo_toml = read_toml(&args.output.join("Cargo.toml"))?;
	let version = Version::parse(&cargo_toml.package.version)?;

	let features_removed = cargo_toml.features.iter().filter(|(k, _)| !features.contains_key(k.to_owned())).next().is_some();
	let features_added = features.iter().filter(|(k, _)| !cargo_toml.features.contains_key(k.to_owned())).next().is_some();
	let content_changed = features_removed || features_added || content_hash != cargo_toml.package.metadata.gitignores.content_hash;

	eprintln!("features removed: {:?}\nfeatures added: {:?}\ncontent hash changed: {:?}", features_removed, features_added, content_changed);
	eprintln!("current version: {}", version);

	let mut new_version = version.clone();
	if features_removed {
		new_version.major += 1;
		new_version.minor = 0;
		new_version.patch = 0;
	} else if features_added {
		new_version.minor += 1;
		new_version.patch = 0;
	} else if content_changed {
		new_version.patch += 1;
	}

	eprintln!("new version: {}", new_version);
	if version == new_version {
		eprintln!("no changes, abort");
		return Ok(());
	}

	let mut new_toml = read_toml(&args.cargo_toml_base)?;
	new_toml.features = features;
	new_toml.package.version = new_version.to_string();
	new_toml.package.metadata.gitignores.content_hash = content_hash;
	new_toml.package.metadata.gitignores.commit = commit;

	let bytes = toml::to_vec(&new_toml)?;
	let len = bytes.len();

	let mut new_cargo_toml = File::create(&args.output.join("Cargo.toml"))?;
	new_cargo_toml.write_all(&bytes)?;
	eprintln!("Cargo.toml: {} bytes written", len);

	Ok(())
}

fn read_toml(path: &Path) -> Result<CargoToml> {
	let mut file = File::open(path)?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	Ok(toml::from_str(&contents)?)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct CargoToml {
	#[serde(default, serialize_with = "toml::ser::tables_last")]
	features: BTreeMap<String, Vec<String>>,

	#[serde(flatten, serialize_with = "toml::ser::tables_last")]
	other: BTreeMap<String, Value>,

	package: Package,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Package {
	#[serde(flatten, serialize_with = "toml::ser::tables_last")]
	other: BTreeMap<String, Value>,

	version: String,

	#[serde(default)]
	metadata: Metadata,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct Metadata {
	#[serde(flatten, serialize_with = "toml::ser::tables_last")]
	other: BTreeMap<String, Value>,

	#[serde(default)]
	gitignores: GitIgnoresMetadata,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct GitIgnoresMetadata {
	commit: String,
	content_hash: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct GitIgnore {
	contents: String,
	file_name: String,
	file_path: String,
	href: String,
	collection: String,
	variant: String,
	feature: String,
}

impl GitIgnore {
	fn new(path: &Path, repo: &Path, commit: &str) -> Result<Self> {
		let rel_path = path.strip_prefix(repo)?;
		let file_path = rel_path.display().to_string();
		assert_eq!(Path::new(&file_path), rel_path);

		let file_name = rel_path.file_name().ok_or_else(|| eyre!("no filename: {:?}", &path))?.to_string_lossy().to_string();
		let mut segs = rel_path.components();

		let mut collection = segs.next().ok_or_else(|| eyre!("no parent: {:?}", &path))?.as_os_str().to_string_lossy().to_string();
		let mut variant = segs.as_path().to_string_lossy().to_string();

		if collection.ends_with(".gitignore") {
			variant = collection;
			collection = String::from("Root");
		}

		let variant = variant.trim_end_matches(".gitignore").replace("#", "Sharp").replace("+", "Plus").to_string();
		let feature = format!("{}-{}", collection, variant);

		Ok(Self {
			contents: std::fs::read_to_string(&path)?,
			href: format!("https://raw.githubusercontent.com/github/gitignore/{}/{}", commit, file_path),
			file_name,
			file_path,
			collection: collection.to_camel_case(),
			variant: variant.to_camel_case(),
			feature: feature.to_kebab_case(),
		})
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Collection {
	name: String,
	feature: String,
	variants: Vec<GitIgnore>,
}

impl Collection {
	fn new(name: &str, variants: impl Iterator<Item=GitIgnore>) -> Self {
		let variants = variants.collect::<Vec<_>>();
		let feature = name.to_kebab_case();

		Self {
			name: name.to_camel_case(),
			feature,
			variants,
		}
	}

	fn generate_module(&self) -> String {
		format!("
		use crate::GitIgnore;

		#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
		pub enum {name} {{ {variants} }}

		impl GitIgnore for {name} {{
			#[cfg(feature = \"no-contents\")]
			fn contents(self) -> &'static str {{
				\"\"
			}}

			#[cfg(not(feature = \"no-contents\"))]
			fn contents(self) -> &'static str {{
				match self {{ {contents} }}
			}}

			fn file_name(self) -> &'static str {{
				match self {{ {file_names} }}
			}}

			fn file_path(self) -> &'static str {{
				match self {{ {file_paths} }}
			}}

			fn list() -> Vec<&'static str> {{
				#[allow(unused_mut)]
				let mut list = Vec::with_capacity({len});
				{list}
				list
			}}
		}}

		#[cfg(all(feature = \"std\", not(feature = \"no-contents\")))]
		impl std::fmt::Display for {name} {{
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
				f.write_str(self.contents())
			}}
		}}
		",
		name=self.name,
		len=self.variants.len(),
		variants=self.variants.iter().map(|v| format!("#[cfg(feature = \"{}\")] {}", v.feature, v.variant)).collect::<Vec<_>>().join(", "),
		contents=self.variants.iter().map(|v| format!("#[cfg(feature = \"{}\")] Self::{} => {:?}", v.feature, v.variant, v.contents)).collect::<Vec<_>>().join(", "),
		file_names=self.variants.iter().map(|v| format!("#[cfg(feature = \"{}\")] Self::{} => {:?}", v.feature, v.variant, v.file_name)).collect::<Vec<_>>().join(", "),
		file_paths=self.variants.iter().map(|v| format!("#[cfg(feature = \"{}\")] Self::{} => {:?}", v.feature, v.variant, v.file_path)).collect::<Vec<_>>().join(", "),
		list=self.variants.iter().map(|v| format!("#[cfg(feature = \"{}\")] list.push({:?});", v.feature, v.variant)).collect::<Vec<_>>().join(" "),
		)
	}

	fn feature_list(&self) -> BTreeMap<String, Vec<String>> {
		let mut map = BTreeMap::new();
		map.insert(self.feature.clone(), self.variants.iter().map(|v| v.feature.clone()).collect::<Vec<_>>());
		for variant in &self.variants {
			map.insert(variant.feature.clone(), Vec::new());
		}
		map
	}
}
