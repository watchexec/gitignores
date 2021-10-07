use std::path::PathBuf;

use color_eyre::eyre::Result;
use dunce::canonicalize;
use structopt::StructOpt;

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

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let mut args = Args::from_args();

    args.output = canonicalize(args.output)?;
    args.cargo_toml_base = canonicalize(args.cargo_toml_base)?;
    args.gitignores_repo = canonicalize(args.gitignores_repo)?;
    if let Some(gh) = args.gh_env_file.take() {
        args.gh_env_file = Some(canonicalize(gh)?);
    }

    dbg!(args);

    Ok(())
}
