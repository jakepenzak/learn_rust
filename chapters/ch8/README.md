# Chapter 8: Common Collections

Rust's standard library includes a number of useful data structures called *collections*. These *collections* point to data stored on the heap, unlike array and tuple types.

- A *vector* allows you to store a variable number of values next to each other.
- A *string* is a collection of characters.
- A *hash map* allows you to assobiate a value with a specific key. A particular implementation of a *map*.

**Table of Contents**
- [8.1 - Storing Lists of Values with Vectors](#81---storing-lists-of-values-with-vectors)
- [8.2 - Storing UTF-8 Encoded Text with Strings](#82---storing-utf-8-encoded-text-with-strings)
- [8.3 - Storing Keys with Associated Values in Hash Maps](#83---storing-keys-with-associated-values-in-hash-maps)

## 8.1 - Storing Lists of Values with Vectors

*Vectors* - `Vec<T>` - allow you to store more than one value (of the same data type) in a single data structure that puts all the values next to each other in memory.

[API Documentation](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html)

### Creating a New Vector

```rust
let v: Vec<i32> = Vec::new();

let v = vec![1, 2, 3];
```

### Updating a Vector

Use `push` method.

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
```

### Reading Elements of Vectors

Use index or `get` method.

```rust
let v = vec![1,2,3,4,5];
let third: &i32 = &v[2];
println!("The third element is {third}");

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}
```

### Iterating Over the Values in a Vector

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{i}");
}

let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

### Using an Enum to Store Multiple Types

We can use a vector of Enums to store multiple types.

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

### Dropping a Vector Drops Its Elements

Like any other `struct`, a vector is freed when it goes out of scope:

```rust
{
    let v = vec![1,2,3,4];

    //do stuff with v
} // <- v goes out of scope and is freed here
```

## 8.2 - Storing UTF-8 Encoded Text with Strings

## 8.3 - Storing Keys with Associated Values in Hash Maps
