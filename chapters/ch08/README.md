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

We discuss strings in the context of collections because strings are implemented asa collection of bytes.

### What is a String?

Rust has only one string type in the core language, which is the string slice `str` that is usuall seen in its borrowed form `&str`.

The `String` type, provided by Rust's standard library, is a growable, mutable, owned, UTF-8 encoded string type.

### Creating a New String

`String` is implemeneted as a wrapper around a vector of bytes, with some extra guarantees, restrictions, and capabilities, and thus has many of the same operations as `Vec<T>`.

```rust
let mut s = String::new();

let data = "initial contents";
let s = data.to_string();
let s = "initial contents".to_string();
let s = String::from("initial contents");
```

### Updating a String

A `String` can grow in size and its contents can change.

#### Appending to a String with `push_str` and `push`

```rust
let mut s = String::from("foo");
s.push_str("bar"); // s = 'foobar' - push_str takes string slice as arg and thus not ownership

let mut s = String::from("lo");
s.push("l"); // s = 'lol'
```

#### Conatenation with the `+` Operator or the `format!` Macro

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1 has ownership moved here and is no longer valid.

let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;
let s = format!{"{s1}-{s2}-{s3}"}; // More readable. This does not take ownership of any parameters.
```

### Indexing into Strings

Rust doesn't support string indexing b/c of the way it stores strings in memory (e.g., UTF-8 encoded and each index represents a byte not a character).

### Slicing Strings

Instead of indexing with a single number, you can slice using a range:

```rust
let hello = "hello";
let s = &hello[0..2]; // s = 'he'
```

### Methods for Iterating Over Strings

Remember valid Unicode scalar values may be made up of more than one byte!

```rust
for c in "LOL".chars() {
    println!("{c}");
}

for b in "LOL".bytes() {
    println!("{b}");
}
```

### Strings Are Not So Simple

See standard library documentation for methods and tools to make working with strings easier. For example, `contains` and `replace` methods.

## 8.3 - Storing Keys with Associated Values in Hash Maps

The `HashMap<K,V>` stores a mapping of keys of type `K` to values of type `V` using a hashing function. Names for this in other languages include *hash*, *object*, *hash table*, *dictionary*, or *associate array*.

### Creating a New Hash Map

Hash Maps are homogenous, like vectors, where all keys and values must be of same type, respectively.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"),10);
scores.insert(String::from("Yellow"),50);
```

### Accessing Values in a Hash Map

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"),10);
scores.insert(String::from("Yellow"),50);

let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);

for (key, value) in &scores {
    println!{"{key}: {value}"};
}
```

### Hash Maps and Ownership

For types that implement `Copy` trait, like `i32`, values are copied into the hash map. For owned values, like `String`, values have ownership moved into Hash Map.

```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value); // At this point field_name and field_value have had ownership moved to map and are invalid.
```

### Updating a Hash Map

#### Overwritting a Value

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{scores:?}"); // Returns {"Blue":25}
```

#### Adding a Key and Value Only If a Key Isn't Present

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"),10);

scores.entry(String::from("Yellow")).or_insert(50); // Since Yellow doesn't exist, this creates record
scores.entry(String::from("Blue")).or_insert(50); // Since Blue exists, this does nothing

println!("{scores:?}");
```

#### Updating a Value Based on the Old Value

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{map:?}");
```

### Hashing Functions

By default, `HashMap` uses a hashing function called *SipHash* that can provide resistence to denial-of-service attacks involving hash tables.
