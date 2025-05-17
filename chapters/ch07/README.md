# Chapter 7: Managing Growing Projects with Packages, Crates, and Modules

Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the module system, include:

- **Packages**: A Cargo feature that lets you build, test, and share crates
- **Crates**: A tree of modules that produces a library or executable
- **Modules** and **use**: Let you control the organization, scope, and privacy of paths
- **Paths**: A way of naming an item, such as a struct, function, or module

**Table of Contents**
- [7.1 - Packages and Crates](#71---packages-and-crates)
- [7.2 - Defining Modules to Control Scope and Privacy](#72---defining-modules-to-control-scope-and-privacy)
- [7.3 - Paths for Referring to an Item in the Module Tree](#73---paths-for-referring-to-an-item-in-the-module-tree)
- [7.4 - Bringing Paths Into Scope with the `use` Keyword](#74---bringing-paths-into-scope-with-the-use-keyword)
- [7.5 - Separating Modules into Different Files](#75---separating-modules-into-different-files)

## 7.1 - Packages and Crates

A *crate* is the smallest amount of code that the Rust compiler considers at a time. Crates can contain modules, and the modules may be defined in other files that get compiled with the crate.

**Two flavors:**

- **Binary crates**: Project that can compile to an executable (e.g., a command-line program, server). Has `main` function.
- **Library crates**: Project that don't compile to an executable and don't have a `main` function. They define functionality intended to be shared with other programs (e.g., `rand`).

The *crate root* is a source file that the Rust compiler starts from and makes up the root module of your crate.

A *package* is a bundle of one or more creates that provides a set of functionality. A package contains a `Cargo.toml` file that describes how to build those crates. A package can contain as many binary crates as you like, but at most only one library crate. A package must contain at least one crate, whether that’s a library or binary crate.

`src/main.rs` => binary crate

`src/lib.rs` => library crate

(A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate)

## 7.2 - Defining Modules to Control Scope and Privacy

*paths* allow you to name items, `use` keyword brings a path into scope, and `pub` keyword makes an item public.

### [Modules Cheat Sheet](https://doc.rust-lang.org/stable/book/ch07-02-defining-modules-to-control-scope-and-privacy.html#modules-cheat-sheet)


- **Start from the crate root**: When compiling a crate, the compiler first looks in the crate root file (usually src/lib.rs for a library crate or src/main.rs for a binary crate) for code to compile.
- **Declaring modules**: In the crate root file, you can declare new modules; say you declare a “garden” module with `mod garden;`. The compiler will look for the module’s code in these places:
  - Inline, within curly brackets that replace the semicolon following `mod garden`
  - In the file *src/garden.rs*
  - In the file *src/garden/mod.rs*
- **Declaring submodules**: In any file other than the crate root, you can declare submodules. For example, you might declare `mod vegetables;` in *src/garden.rs*. The compiler will look for the submodule’s code within the directory named for the parent module in these places:
  - Inline, directly following `mod vegetables`, within curly brackets instead of the semicolon
  - In the file *src/garden/vegetables.rs*
  - In the file *src/garden/vegetables/mod.rs*
- **Paths to code in modules**: Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. For example, an `Asparagus` type in the garden vegetables module would be found at `crate::garden::vegetables::Asparagus`.
- **Private vs. public**: Code within a module is private from its parent modules by default. To make a module public, declare it with `pub mod` instead of `mod`. To make items within a public module public as well, use `pub` before their declarations.
- **The `use` keyword**: Within a scope, the use keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to `crate::garden::vegetables::Asparagus`, you can create a shortcut with `use crate::garden::vegetables::Asparagus;` and from then on you only need to write `Asparagus` to make use of that type in the scope.

See [*backyard/*](backyard/) for a simple example.

### Grouping Related Code in Modules

See [*restaurant*](restaurant/) for a concrete example. The resulting module structure looks the this:

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

## 7.3 - Paths for Referring to an Item in the Module Tree

To show Rust where to find an item in a module tree, we use a path in the same way we use a path when navigating a file system. Nothing fancy here.

Can use *absolute* paths, which start from a crate root, or *relative* paths, which start from the current module.

Prefer *absolute* generally.

We use `::` to separate identifiers in the path.

### Exposing Paths with the `pub` Keyword

By default, all items (functions, methods, structs, enums, modules, and constants) are private to parent modules by default.

See [*restaurant*](restaurant/) for a concrete example. This works b/c, while `front_of_house` isn’t public, because the `eat_at_restaurant` function is defined in the same module as `front_of_house` (that is, `eat_at_restaurant` and `front_of_house` are siblings), we can refer to `front_of_house` from `eat_at_restaurant`. Next is the `hosting` module marked with `pub`. We can access the parent module of `hosting`, so we can access `hosting`. Finally, the `add_to_waitlist` function is marked with `pub` and we can access its parent module, so this function call works!

If planning on making a library public, refer to [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/about.html).

### Starting Relative Paths with `super`

`super` refers to the parent module, similar to using `..` in a file system path. See `super_import` function in restaurant.

### Making Structs and Enums Public

In general, we can use `pub` to make structs and enums public, but there are a few caveats.

1. If we use `pub` before a struct definition, we make the struct public, but the struct's fields will still be private. We can make individual fields public by using `pub` before each field in the struct definition.
1. If we use `pub` before an enum definition, we make the enum public and all its variants will be public as well.

## 7.4 - Bringing Paths into Scope with the `use` Keyword

It is overly verbose to have to specify full path to use each function (e.g., `let mut meal = back_of_house::Breakfast::summer("Rye");`). Luckily, we can easily get around this using the `use` keyword.

Adding `use` and a path in scope is similar to creating symbolic link.

`use` only brings into scope the shortcut into the scope it is defined.

### Creating Idiomatic `use` Paths

When brining in a function, the idiomatic way is to bring the parent module into scope, rather than function itself. That is,

```rust
// Both work
use crate::front_of_house::hosting; // This is idiomatic
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(); // This is idiomatic
    add_to_waitlist();
}
```

On the other hand, when using structs, enums, or other items, it is idiomatic to bring the whole path into scope. The exception is any conflicting names, obviously.

### Providing New NAmes with the `as` Keyword

Similar to python, we can change name of import as:

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

### Re-exporting Names with `pub use`

We can "rexport" a module at point of `use` and enable other callers of that module to access it from module in which `use` is defined.

```rust
// lib.rs (restuarant lib)
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

Now I'd be able to access `hosting` via `restaurant::hosting` instead of `crate::front_of_house::hosting`.

Similar to using `__init__.py` in python to reorganize imports, we can use `pub use` to re-export names from a module to make them available to external code.

### Using External Packages

Packages are publicly available at [crates.io](https://crates.io/). To use them, you first add to `Cargo.toml` and then import them with `use`:

```toml
rand = "0.8.5"
```

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);
}
```

`std` is standard library that comes with Rust language by default.

### Using Nested Paths to Clean Up Large `use` Lists

When importing multiple items from the same crate or module, we can reduce verbosity of `use` lists using nested paths:

```rust
use std::{cmp::Ordering, io};
use std::io::{self, Write};
```

### The Glob Operator

As in python, avoid if possible, due to ambiguity and namespacing clashes (although Rust protects against the latter?)

```rust
use std::collections::*;
```

## 7.5 - Separating Modules into Different Files

The examples thus far in restaurant have all modules defined within the same `lib.rs` script. In realistic scenarios, we will want to break these modules out into files.

See the `front_of_house` examples and compare to `back_of_house` that is within `lib.rs`.
