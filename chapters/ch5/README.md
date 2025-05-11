# Chapter 5: Using Structs to Structure Related Data

A struct, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group. If you’re familiar with an object-oriented language, a struct is like an object’s data attributes.

Structs and enums (discussed in Chapter 6) are the building blocks for creating new types in your program’s domain to take full advantage of Rust’s compile-time type checking.

For immutable and mutable structs, this is very similar to NamedTuples and DataClasses in python, respectively.

**Table of Contents**
- [5.1 - Defining and Instantiating Structs](#51---defining-and-instantiating-structs)
- [5.2 - An Example Program Using Structs](#52---an-example-program-using-structs)
- [5.3 - Method Syntax](#53---method-syntax)

## 5.1 - Defining and Instantiating Structs

Structs are similar to tuples, but more flexible.

```rust
struct User{
    active: bool, // fields
    username: String,
    email: String,
    sign_in_count: u64
}
```

To create an instance (struct itself is either mutable or not, not the fields),

```rust
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```

Accessing a value from a struct instance is done using dot notation:

```rust
    user1.email = String::from("anotheremail@example.com");

    println!("User email: {}", user1.email);
```

Using in a function, with type hints:

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
```

### Using the Field Init Shorthand

Because username is already a variable name, we can use the field init shorthand to create a new User instance.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

### Creating Instances from Other Instances with Struct Update Syntax

```rust
fn main() {
    // --snip--
    //
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // Sort of like kwargs in python
    }
}
```

> [!WARNING]
> Fields in user1 that don't implement `Copy` have ownership transferred to user2. Those fields are now inaccessible from user1.

### Using Tuple Structs Without Named Fields to Create Different Types

*Tuple structs* are similar to tuples, but they have a name and can be used to create different types (fields are not named).

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

Each *struct* or *tuple struct* you define becomes it's own type.

### Unit-Like Structs Without Any Fields

*Unit-like structs* are structs that don't have any fields. They are useful for creating types that don't have any data associated with them.

```rust
struct AlwaysEqual;

let unit_like_struct = AlwaysEqual;
```

> [!WARNING]
> References owned by something else can not be stored in structs without the use of *lifetimes*. See chapter 10.

## 5.2 - An Example Program Using Structs

See [`rectangles/`](rectangles/)

Using structs to create program that computes area of rectangle.

- Demonstrated naive case, using a simple function and 2 variables/arguments of height and width
- Demonstrated tuple case, using a function and 1 variables/argument of tuple referenced by indices
- Demonstrated struct case, using a function and 1 variables/argument of struct referenced by fields
  - Added derived traits (`Debug`) for helpful printing functionality


## 5.3 - Method Syntax

See continutation of [`rectangles/`](rectangles/).

*Methods* are similar to functions, but defined within the context of a struct.

This is analogous to classes and methods in Python.

First argument is borrowing itself via `&self`, which is an alias for `self: &Self`. Methods can take ownership of `self`, borrow `self` immutable as done in example, or borrow `self` mutably.

Methods can take any other parameters like a function.

*Associated functions* are similar to methods, but they don't have an instance of the struct to work with. They are defined within the context of a struct, but they don't take `self` as a parameter. To call this associated function, we can use the `::` syntax, which is used for both associated functions and namespaces created by modules.

You can also have multiple `impl` blocks, but this is typically useful only for generic types and traits.
