# Chapter 10: Generic Types, Traits, and Lifetimes

*Generics* are abstract stand-ins for concrete types or other properties. One such example being `Option<T>`, where `<T>` represents the generic.

*Traits* define behavior in a generic way. For example, we can combine traints with generic types to constrain the generic type to accept only those types that have a particular behavior.

*Lifetimes* are a variety of generics that give the compiler information about how references relate to each other.

**Table of Contents**
- [10.1 - Generic Data Types](#101---generic-data-types)
- [10.2 - Traits: Defining Shared Behavior](#102---traits-defining-shared-behavior)
- [10.3 - Validating References with Lifetimes](#103---validating-references-with-lifetimes)

## 10.1 - Generic Data Types
We use generics to create definitions for items like function signatures or structs, which we can then use with many concrete data types.

### In Function Definitions

```rust
fn largest<T>(list: &[T]) -> &T {
    lut mut largest = &list[0];

    for iterm in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main () {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

**This example won't work!** - We need to further restrict what `T` can be (e.g., it needs a specific *trait*).

### In Struct Definitions

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}

struct Point<T, U> { // For two different types
    x: T,
    y: U,
}
```

### In Enum Definitions

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### In Method Definitions

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}
```

Note that we need to specificy `impl<T>` to let compiler know that `Point<T>` is referring to generic type not concrete. This is b/c we could have a method implemented only for a specific type `T` in `Point<T>`:

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

The generic type parameters in a struct definition aren't always the same as those you us in that same struct's method signatures. Take the following example:

```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

### Performance of Code Using Generics

There should be *no* performance penalty for using generics. The Rust compiler uses a process called *monomorphization* to turn generic code into specific code at compile time. This means that the code you write using generics is not slower than the same code with only concrete types.

## 10.2 - Traits: Defining Shared Behavior

A *trait* defines the functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use *trait bounds* to specify that a generic type can be any type that has certain behavior.

### Defining a Trait

A type's behavior consists of the methods we can call on that type. Different types share the same behavior if we can call the same methods on all of those types. Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomlish some purpose.

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

The compiler will enforce that any type that has the `Summary` trait will have the method `summarize` defined with this signature exactly. This is conceptually similar to `Abstract Base Classes` in python.

### Implementing a Trait on a Type

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

In order to use this trait, we can call `summarize` method, but we must also bring the trait into scope!

```rust
use aggregator::{SocialPost, Summary};

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };
    println!("{}", post.summarize());
}
```

### Default Implementations

When defining a trait, we can also add default behavior!

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Summary for NewsArticle {}

impl Summary for SocialPost {}
```

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

### Traits as Parameters

We can define a function that takes a parameter that implements the `Summary` trait:

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

#### Trait Bound Syntax

`impl Trait` syntax is actually syntax sugar for a longer form known as a *trait bound*:

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

Choosing between both options depends on complexity:

```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {} // Only requires args to implement Summary

pub fn notify<T: Summary>(item1: &T, item2: &T) {} // Requires args to both implement Summary and be of same type T
```

#### Specifying Multiple Trait Bounds with `+` Syntax

We can also specifiy more than one trait bound:

```rust
pub fn notify(item: &(impl Summary + Display)) {}
pub fn notify<T: Summary + Display>(item: &T) {}
```

#### Clearer Trait Bounds with `where` Clauses

In some cases, trait bounds can make function signature very verbose, we can improve this:

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u:&U) -> i32 {}

fn some_function<T, U>(t: &T, u:&U) -> i32
where
    T: Dsiplay + Clone,
    U: Clone + Debug,
{}
```

### Returning Types that Implement Traits

```rust
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    }
}
```

This only works when function returns a single type. Solutions to this are discussed in [ch. 18](../ch18).

### Using Trait Bounds to Conditionally Implement Methods

```rust
use std::fmt::Display;

struct Pair<T> {
   x: T,
   y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> { // Only implements method if T has both traits
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

We can also conditionally implement a trait for any type that implements another trait, known as *blanket implementations*:

```rust
impl<T: Dsiplay> ToString for T {}
```

## 10.3 - Validating References with Lifetimes
