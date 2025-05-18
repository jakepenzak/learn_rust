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

*Lifetimes* are another generic that ensures the references are valid as long as we need them to be.

Every reference in Rust has a lifetime, which is the scope for which that reference is valid. Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred. We only have to annotate types when multiple types are possible. In a similar way, we have to annotate lifetimes when the lifetimes of references could be related in a few different ways. Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

See original chapter to understand how Rust checks lifetimes using *The Borrow Checker*.

### Generic Lifetimes in Functions

The following example doesn't work b/c the compiler doesn't which input reference the return is a reference to!

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(),string2);
    println!("The longest string is {}", result);
}

fn longest(x:&str, y: &str)-> &str {
    if x.len() > y.len() { x } else { y }
}
```

### Lifetime Annotation Syntax

Lifetime annotations don't change how long any of the references they live, but rather describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.

```rust
&i32 // a reference
&'a // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

### Lifetime Annotations in Function Signatures

We want the expression to express the following constraint: the return reference will be valid as long as both the parameters are valid.

```rust
fn longest<'a>(x:&'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

The function signature now tells Rust that for some lifetime `'a`, the function takes two parameters, both of which are string slices that live at least as long as lifetime `'a`. The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime `'a`. In practice, it means that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the values referred to by the function arguments. These relationships are what we want Rust to use when analyzing this code.

The return lifetime must have a corresponding lifetime parameter in the function signature.

### Lifetime Annotations in Struct Definitions

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt{ part: first_sentence, };
}
```

### Lifetime Elision

Historically, lifetimes were needed for all references in a function signature. However, Rust has a set of rules called *lifetime elision* that allow it to infer the lifetimes of references in certain cases:

The compiler will go through each rule and try to infer lifetimes of references:

1. Each parameter that is a reference get;s assigned one lifetime parameter.
2. If there is exactly one input lifetime paremeter, that lifetime is assigned to all output lifetime parameters.
3. If there are multiple input lifetimte parameters, but one of them is `&self` or `&mut self`, the lifetime of `self` is assigned to all output lifetime parameters.

See chapter for more details on lifetimes in methods.

### The Static Lifetime

In certain cases, you may want the lifetime of a reference to be the entire duration of the program. This is called the `'static` lifetime. The lifetime of all string literals is static.

```rust
let s: &'static str = "I have a static lifetime";
```

### Combining All of Ch. 10

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len()  > y.len() { x } else { y }
}
```
