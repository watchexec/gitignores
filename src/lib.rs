//! Each gitignore is available as a variant of one of three enums:
//!
//! ```
//! dbg!(
//!     gitignores::Root::Rust,
//!     gitignores::Global::Emacs,
//!     gitignores::Community::Racket,
//! );
//! ```
//!
//! The enums implement [`Display`](std::fmt::Display) / `.to_string()`, which will return the
//! contents of the gitignore file (only when the `std` feature is enabled, and the `no-contents`
//! feature disabled):
//!
//! ```
//! println!("{}", gitignores::Root::Rust);
//! gitignores::Global::Emacs.to_string();
//! ```
//!
//! The enums also implement a [`GitIgnore`] trait:
//!
//! ```
//! trait GitIgnore {
//!     /// The contents of the gitignore
//!     ///
//!     /// Returns an empty string if the `no-contents` feature is enabled.
//!     fn contents(self) -> &'static str;
//!
//!     /// The file name of the gitignore
//!     fn file_name(self) -> &'static str;
//!
//!     /// The full path of the gitignore relative to repo root
//!     fn file_path(self) -> &'static str;
//!
//!     /// The list of all included gitignores
//!     fn list() -> Vec<&'static str>;
//! }
//! ```
//!
//! Finally, there is a constant with the git reference of the commit the crate was built from:
//!
//! ```
//! dbg!(gitignores::GIT_COMMIT_REF);
//! ```
//!
//! This is also available in the `package.metadata.gitignores` table in the Cargo manifest.
//!
//! ## Features
//!
//! By default all gitignores are included, but you can customise this as granularily as you wish.
//! To get started with selecting your custom set, first disable the default features:
//!
//! ```toml
//! [dependencies.gitignores]
//! default-features = false
//! features = []
//! ```
//!
//! ### Collections
//!
//! | Feature name | Path in github/gitignore repo | Path in crate |
//! |:-------------|:------------------------------|:--------------|
//! | `root`       | `/*.gitignore`                | [`Root`]      |
//! | `global`     | `/Global/**/*.gitignore`      | [`Global`]    |
//! | `community`  | `/community/**/*.gitignore`   | [`Community`] |
//!
//! ### Individual gitignores
//!
//! Each gitignore can be enabled with the `<collection>-<name>` feature. Gitignores in subfolders
//! have the folder name prepended to the name, like `<collection>-<folder>-<name>`, all lowercase.
//!
//! ### Other
//!
//! - `no-contents`: omit the embedded file contents, leaving only the metadata.
//! - `std`: implement the `Display` trait on the enums (except without `no-contents`).
//!
//! ### Examples
//!
//! #### All globals and only Rust root
//!
//! ```toml
//! [dependencies.gitignores]
//! default-features = false
//! features = ["global", "root-rust"]
//! ```
//!
//! #### Some specific gitignores
//!
//! ```toml
//! [dependencies.gitignores]
//! default-features = false
//! features = ["community-racket", "global-emacs", "root-commonlisp"]
//! ```
//!
//! ## Versioning
//!
//! This crate respects semver!
//!
//! It will bump the major version (breaking release) when:
//! - Gitignores disappear from a collection
//! - Gitignores move from a collection to another
//! - Gitignores are renamed
//! - The minimum required Rust version increases
//!
//! It will bump the minor version when:
//! - New gitignores are added to a collection
//!
//! It will bump the patch version when:
//! - Gitignore contents change

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::vec_init_then_push)]

#[doc(inline)]
pub use common::*;
#[doc(inline)]
pub use gitref::*;

#[doc(inline)]
pub use community::*;
#[doc(inline)]
pub use global::*;
#[doc(inline)]
pub use root::*;

mod common;
mod gitref;

mod community;
mod global;
mod root;
