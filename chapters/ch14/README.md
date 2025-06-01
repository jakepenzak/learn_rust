# Chapter 14: More About Cargo and Crates.io

Coverage of more advanced features w/ cargo:
- Customize your build through release profiles
- Publish libraries on [crates.io](https://crates.io/)
- Organize large projects with workspaces
- Install binaries from [crates.io](https://crates.io/)
- Extend Cargo using custom commands

For more on cargo, see [Cargo Book](https://doc.rust-lang.org/cargo/).

**Table of Contents**
- [14.1 - Customizing Builds with Release Profiles](#141-customizing-builds-with-release-profiles)
- [14.2 - Publishing a Crate to Crates.io](#142-publishing-a-crate-to-cratesio)
- [14.3 - Cargo Workspaces](#143-cargo-workspaces)

## 14.1 - Customizing Builds with Release Profiles

In Rust, *release profiles* are predefined and customizable profiles with different configurations that allow a programmer to have more control over various options for compiling code.

Two main profiles: `dev` (default) and `release` via `cargo build` and `cargo build --release`, respectively.

Each have different defaults. For example, `opt-level` controls the level of optimization applied to the code (from 0-3), where `dev` defaults to 0 (minimal optimizations, fastest build time) and `release` defaults to 3 (maximum optimizations, slower build time).

Many more configuration options exist, see docs.

## 14.2 - Publishing a Crate to Crates.io

### Making Useful Documentation Comments

Documentation comments use three slashes `///` instead of two, which renders to html in documentation and supports Markdown notation. Documentation comments should be placed just beore the item they're documenting.

```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

We can then generate and open docs via `cargo doc --open`.

#### Commonly Used Sections

- **Examples** - shown above
- **Panics** - Scenarios where the function will panic
- **Errors** - Errors that can be returned by the function, if the function returns a `Result`
- **Safety** - If the function is unsafe, explain why the function is unsafe

#### Documentation Comments as Tests

When running tests, all examples in documentation comments get tested too!

#### Commenting Contained Items

The style of doc comment `//!` adds documentation to the item that contains the comments rather than to the items following the comments. This is typically done to document modules, crates, and other items that contain other items.

For example, in *src/lib.rs*, we can include:

```rust
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--
```

### Exporting a Convenient Public API with `pub use`

We can call `pub use` to re-export items from a private or deeply nested module to make them public and accessible higher in the tree. For example, in *src/lib.rs*, we can call:

```rust
use art::kinds::PrimaryColor;
use art::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
```

### Setting Up a Creates.io Account

See [here](https://doc.rust-lang.org/stable/book/ch14-02-publishing-to-crates-io.html#setting-up-a-cratesio-account).

### Adding Metadata to a New Crate

See [here](https://doc.rust-lang.org/stable/book/ch14-02-publishing-to-crates-io.html#adding-metadata-to-a-new-crate).

### Publishing to Crates.io

See [here](https://doc.rust-lang.org/stable/book/ch14-02-publishing-to-crates-io.html#publishing-to-cratesio)

### Publishing a New Version of an Existing Crate

See [here](https://doc.rust-lang.org/stable/book/ch14-02-publishing-to-crates-io.html#publishing-a-new-version-of-an-existing-crate).

### Deprecating Versions from Crates.io with `cargo yank`

## 14.3 - Cargo Workspaces

Useful for breaking apart library crates into multiple crates.

See docs for more details.

### Creating a Workspace

A *workspace* is a set of packages that share the same *Cargo.lock* and output directory.

Creates in a *workspace* are meant to depend on each other and thus are compiled together.

### Adding a Test to a Workspace

From top-level directory, we can test all creates at once or individuall with `-p` flag.

For publishing, we must still publish each crate individually.

## 14.4 - Installing Binaries from Crates.io with `cargo install`

Self explanatory - see docs.

## 14.5 - Extending Cargo with Custom Commands

See docs.
