# Chapter 1: Getting Started with Rust

## Introduction

- Rust balances high-level ergonomics with low-level control.

- Contemporary Developer Tools Out-of-the-box
  - Cargo, the Rust package manager and build tool
  - Rustfmt, the Rust code formatter
  - rust-analyzer, the Rust language server
  - clippy, the Rust linter
  - rust-doc, the Rust documentation generator
  - rustup, the Rust toolchain manager
  - rustc, the Rust compiler
  - rust-std, the Rust standard library


### Installation

See [here](https://doc.rust-lang.org/stable/book/ch01-01-installation.html)

`rustup -- version` - Check your Rust installation
`rustup update` - Update your Rust installation

### Hello, World!

See [`hello_world/`](hello_world/)

- Compile - `rustc main.rs`
- Run as executable - `./main`

### Hello, Cargo!

**Best way to create any Rust project is to use Cargo.**

See [`hello_cargo/`](hello_cargo/)

- Create Project - `cargo new hello_cargo`
- Build Project - `cargo build`
- Build & Run Project - `cargo run`
- Check it can compile - `cargo check`
