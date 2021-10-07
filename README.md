[![Crate release version](https://flat.badgen.net/crates/v/gitignores)][crate]
[![Crate license: CC0-1.0](https://flat.badgen.net/badge/license/CC0-1.0)][copyright]
![MSRV: 1.31.0 (breaking)](https://flat.badgen.net/badge/MSRV/1.31.0%20%28breaking%29/green)
[![CI status](https://github.com/watchexec/gitignores/actions/workflows/check.yml/badge.svg)](https://github.com/watchexec/gitignores/actions/workflows/check.yml)

# Gitignores

_GitHub’s collection of gitignores, embedded, automatically updated._

- **[API documentation][docs]**.
- [Public Domain][copyright] via CC0-1.0 (same as [source data][gh-gitignore]).
- **MSRV: 1.31.0** (the first release in the 2018 edition)
- Doesn't require std.
- No dependencies.
- Works offline.

[crate]: https://lib.rs/crates/gitignores
[copyright]: https://creativecommons.org/publicdomain/zero/1.0/
[docs]: https://docs.rs/gitignores
[gh-gitignore]: https://github.com/github/gitignore

## Quick start

```toml
[dependencies]
gitignores = "1.0.0"
```

```rust
dbg!(gitignores::Root::Rust);
```

## API

Each gitignore is available as a variant of one of three enums:

```rust
(
	gitignores::Root::Rust,
	gitignores::Global::Emacs,
	gitignores::Community::Racket,
)
```

The enums implement `Display` / `.to_string()`, which will return the contents of the gitignore
(only when the `std` feature is enabled):

```rust
println!("{}", gitignores::Root::Rust);
gitignores::Global::Emacs.to_string();
```

The enums also implement a `GitIgnore` trait:

```rust
trait GitIgnore {
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
```

Finally, there is a constant with the git reference of the commit the crate was built from:

```rust
dbg!(gitignores::GIT_COMMIT_REF);
```


## Features

By default all gitignores are included, but you can customise this as granularily as you wish. To
get started with selecting your custom set, first disable the default features:

```toml
[dependencies.gitignores]
default-features = false
features = []
```

### Collections

| Feature name | Path in [gitignore][gh-gitignore] repo | Path in crate |
|:-------------|:---------------------------------------|:--------------|
| `root`       | `/*.gitignore`                         | `Root::`      |
| `global`     | `/Global/**/*.gitignore`               | `Global::`    |
| `community`  | `/community/**/*.gitignore`            | `Community::` |

### Individual gitignores

Each gitignore can be enabled with the `<collection>-<name>` feature. Gitignores in subfolders have
the folder name prepended to the name, like `<collection>-<folder>-<name>`. All are lowercased.

### Other

- `no-contents`: omit the embedded file contents, leaving only the metadata.
- `std`: implement the `Display` trait on the enums.

### Examples

#### All globals and only Rust root

```toml
[dependencies.gitignores]
default-features = false
features = ["global", "root-rust"]
```

#### Some specific gitignores

```toml
[dependencies.gitignores]
default-features = false
features = ["community-racket", "global-emacs", "root-commonlisp"]
```

## Versioning

This crate respects semver!

It will bump the major version (breaking release) when:
- Gitignores disappear from a collection
- Gitignores move from a collection to another
- Gitignores are renamed
- The minimum required Rust version increases

It will bump the minor version when:
- New gitignores are added to a collection

It will bump the patch version when:
- Gitignore contents change

This repo [checks for updates](https://github.com/watchexec/gitignores/actions/workflows/update.yml)
to the [gitignore repo][gh-gitignore] once a day, and automatically releases if changes are found.

## License

The published crate ([`gitignores`][crate]) is generated from the data, which is CC0-1.0, and so it
itself is CC0-1.0 (Public Domain).

The generator code is [Apache 2.0 / MIT](updater/COPYRIGHT).
