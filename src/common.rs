/// Common methods for all gitignore collection enums
///
/// You can also use this trait to accept any gitignore type:
///
/// ```
/// use std::path::Path;
/// use gitignore::Gitignore;
/// fn ignored(file: impl AsRef<Path>, gitignore: impl Gitignore) -> bool { false }
/// ```
pub trait GitIgnore {
	/// The contents of the gitignore
	///
	/// Returns an empty string if the `no-contents` feature is enabled.
	fn contents(self) -> &'static str;

	/// The file name of the gitignore
	fn file_name(self) -> &'static str;

	/// The full path of the gitignore relative to repo root
	fn file_path(self) -> &'static str;

	/// The list of all included gitignores
	fn list() -> Vec<&'static str>;
}
