# Chapter 3: Common Programming Concepts

**Table of Contents**
- [3.1 - Variables and Mutability](#31---variables-and-mutability)
- [3.2 - Data Types](#32---data-types)
- [3.3 - Functions](#33---functions)
- [3.4 - Comments](#34---comments)
- [3.5 - Control Flow](#35---control-flow)

## 3.1 - Variables and Mutability

See [`variables/`](variables/)

By default, variables are immutable.
  - Immutable - `let x = 5`
  - Mutable - `let mut x = 5`

### Constants
Constants are always immutable and must be type annotated. Convention is to use all caps with underscores between words.

`const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;`

### Shadowing
Can always re-declare a variable with the same name (including different types, differing it from mutability), which overwrites previous value until overwritten again or scope ends.

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
}
```
## 3.2 - Data Types

Rust is statically typed, meaning compiler must know types at compile time. Compiler can typically infer
types from usage, but in ambiguous cases, it must be explicit.

```rust
// Some examples

let x: i32 = 32;
let x: f32 = 6.23;
let x: f64 = 6.23;
let f: bool = false;
let c = 'z';
let z: char = 'ℤ'; // with explicit type annotation
let heart_eyed_cat = '😻';
```

### Scalar Types

1. Integers

|Size|Signed|Unsigned|
|----|----|------|
|8 bits|`i8`|`u8`|
|16 bits|`i16`|`u16`|
|32 bits|`i32`|`u32`|
|64 bits|`i64`|`u64`|
|128 bits|`i128`|`u128`|
|Platform-dependent|`isize`|`usize`|

2. Floating-Point Numbers

All floating points are signed.

- `f32`
- `f64` (Default)

*Numeric Operations*
```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}
```

3. Booleans

- `bool`

4. Characters

Character literals differ from string literals in that they represent a single Unicode scalar value, while string literals represent a sequence of Unicode scalar values. Character literals are enclosed in single quotes, while string literals are enclosed in double quotes.

- `char`
### Compound Types

1. **Tuple**

A *tuple* is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

`let tup: (i32, f64, u8) = (500, 6.4, 1);`

```rust
// Accessing tuple elements
let tup = (500, 6.4, 1);
let (x, y, z) = tup;
println!("The value of y is: {y}");

let x: (i32, f64, u8) = (500, 6.4, 1);
let five_hundred = x.0;
let six_point_four = x.1;
let one = x.2;
}
```

Empty tuple `()` is called a unit and is default return type for functions that don't return any meaningful value.

2. **Array**

Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.

`let a = [1, 2, 3, 4, 5];`

`let a: [i32; 5] = [1, 2, 3, 4, 5];`

```rust
// Accessing array elements
let a = [1, 2, 3, 4, 5];
let first = a[0];
let second = a[1];
```

```rust
// Bad example of accessing invalid index
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
```

## 3.3 - Functions

See [`functions/`](functions/)

The `main` function is the entry point of a Rust program.
Functions and variables use snake_case conventons.

### Parameters

In function signatures, you *must* declare the type.

```rust
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```

### Statements and Expressions

- Statements are instructions that perform some action and do not return a value.
  - `let y = 6;`
  - `fn ...`
- Expressions evaluate to a resultant value.
  - Calling a function
  - Calling a macro
  - A new scope block

Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.

### Functions with Return Values

You must specify return type after the function signature.

```rust
fn five() -> i32 {
    5 // or return 5
}
```

## 3.4 - Comments

Comments in rust follow the form `// a comment`

## 3.5 - Control Flow

See [`control_flow/`](control_flow/)

- **`if` Expressions** - self explanatory conditional logic
- **`loop` Loops** - similar to `while True` in `while` loops
  - `break` can be used to escape innermost loop
  - Loop labels can help disambiguate `break` usage
- **`while` Loops** - self explanatory
- **`for` Loops** - self explanatory

## Exercises

See [`ch_exercises/`](ch_exercises/)

1. Convert temp between Farenheit and Celsius
2. Generate the nth Fibonacci number
