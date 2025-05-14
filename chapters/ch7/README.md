# Chapter 7: Managing Growing Projects with Packages, Crates, and Modules

Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the module system, include:

- **Packages**: A Cargo feature that lets you build, test, and share crates
- **Crates**: A tree of modules that produces a library or executable
- **Modules** and **use**: Let you control the organization, scope, and privacy of paths
- **Paths**: A way of naming an item, such as a struct, function, or module

**Table of Contents**
- [7.1 - Packages and Crates](#71---packages-and-crates)
- [7.2 - Defining Modules to Control Scope and Privacy](#42---defining-modules-to-control-scope-and-privacy)
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

## 7.3 - Paths for Referencing an Item in the Module Tree

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

    pub enum Appetizer {
        Soup,
        Salad,
    }

## 7.4 - Bringing Paths into Scope with the `use` Keyword
