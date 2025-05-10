# Chapter 4: Understanding Ownership

Rust's most unique feature, with deep inplications for the rest of the language.

**Table of Contents**
- [4.1 - What is Ownership?](#41---what-is-ownership?)
- [4.2 - References and Borrowing](#42---references-and-borrowing)
- [4.3 - The Slice Type](#43---the-slice-type)

## 4.1 - What is Ownership?

*Ownership* is a set of rules that govern how a Rust program manages memory.

Different Paradigms
- *Garbage Collection* - Automatic memory management (e.g., Python, JavaScript, Ruby)
- *Manual Memory Management* - Explicit memory allocation and deallocation (e.g., C, C++)
- **Ownership** - Memory management with explicit ownership that compiler checks. No runtime implications. (Rust)

See discussion on stack vs. heap. In short,

- *Stack* stores values in the order it gets them and removes in opposite order (LIFO). All data stored on stack must have a known, fixed size at compile.
- *Heap* is less organized. It is a pool of memory that is not organized in any particular way. When you put data on te heap, you request a certain amount of space.

Adding & accessing data on a stack is quicker.

**In Rust:**
> When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.

> Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses. Once you understand ownership, you won’t need to think about the stack and the heap very often, but knowing that the main purpose of ownership is to manage heap data can help explain why it works the way it does.

### Ownership Rules

- Each value in Rust has an *owner*
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

### Variable Scope

A scope is the range within a program for which an item is valid. Nothing fancy here.

```rust
{                      // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward

    // do stuff with s
}                      // this scope is now over, and s is no longer valid
```

### The `String` Type

Other basic types (coverd in [ch .3](../ch3)) are already of known size (e.g., `i32`) and can thus be stored on stack.

string literal is immutable, `String` is mutable and stored on heap.\

```rust
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // This will print `hello, world!
```

### Memory and Allocation

1. Requests memory during variable assignment (mostly universal)
2. Return memory to allocator (Rust differs here)
  - Garbage collector keeps track of and cleans up memory that isn't being used anymore
  - W/o GC, we must manually return memory to allocator (reverse of assigment)
  - In Rust, memory is automatically returned once the variable that owns it goes out of scope.

```rust
{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}                                  // this scope is now over, and s is no longer valid`
```

#### Variables and Data Interacting with Move
- Data on stack (like `i32`) stores data directly on stack & data gets copied when assigning to another variable
- Data on heap (like `String`) stores pointer, len, capacity & this would in theory copied when assigning to another variable. Actualy data in heap is not copied. Sort of like a hard link, shallow vs. deep copy.
  - However, calling `drop` on both of these at end of scope would introduce *double free* error.
  - To get around this, Rust invalidates first variable definition when copying - it becomes a "move". Rust will never automatically create a deep copy.

```rust
    let s1 = String::from("hello");
    let s2 = s1; // overwrites s1

    println!("{s1}, world!"); // this wont work
```

#### Scope and Assignment

- When you assigned a completely new value to an existing variable, Rust will call `drop` immediately.

```rust
    let mut s = String::from("hello");
    s = String::from("ahoy"); // calls drop on og s

    println!("{s}, world!");
```

#### Variables and Data Interacting with Clone

If we do want to create a deep copy, we can use `clone` method.

```rust
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
```

#### Stack-Only Data: Copy

Types with `Copy` trait are stored on stack and copied when assigning to another variable.

```rust
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
```

Here are some of the types that implement Copy:

    - All the integer types, such as u32.
    - The Boolean type, bool, with values true and false.
    - All the floating-point types, such as f64.
    - The character type, char.
    - Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

### Ownership and Functions

Mechanics of passing a value to a function are similar to assign a value to a variable.

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // because i32 implements the Copy trait,
                                    // x does NOT move into the function,
    println!("{}", x);              // so it's okay to use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
```

### Return Values and Scopes

Return values can also transfer ownership.

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable.

But, what if we want to use a variable in a function and again later? Do we have to return it every time? This is where *references* come into play.

## 4.2 - References and Borrowing

A *reference* solves the above issue by having as a pointer to the data on the heap, rather than moving the variable and changing ownership itself. However, a reference is guaranteed to point to a valid value for the life of that reference. The action of creating a reference is called *borrowing*.

You can create a reference by prefixing variable with `&` (e.g., `&variable_name`).

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize { // s is a reference String
    s.len()
} // Here s goes out of scope. But because s does not have ownership of what it refers to, the value is not dropped.
```

You can not modify a borrowed value. If you want to modify a value, you need to create a mutable reference.

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

You can not have multiple mutable references to the same data at the same time (e.g., within the same scope). You can have multiple immutable references to the same data at the same time.

A reference's scope starts from where it is introduced and continues through the last time that reference is used.

### Dangling References

A dangling reference is a reference that points to a location in memory that has been freed. This is a problem because the reference is still valid, but the memory it points to has been deallocated. Rust prevents dangling references from happening by not allowing references to be created that point to invalid memory!

Below example is an example of a dangling reference, which will fail to compile.
```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s // Reference s
} // But s gets dropped here, so the reference is dangling!
```

### The Rules of References

- At any given time, you can have *either* one mutable reference *or* any number of immutable references.
- Reference must always be valid.

## 4.3 - The Slice Type

*Slices* let you reference a contiguous sequence of elements in a collectio nrather than the whole collection. A slice is a kind of reference, so it does not have ownership.

### String Slices

A *string slice* is a reference to a portion of a `String`. It looks like:

```rust
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
```

Rewriting `first_word` motivating example:

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

### String Literals as Slices

String literals are slices. The type of `s` in the example below is `&str`:

```rust
let s = "Hello, world!";
```

### String Slices as Parameters

String slices can be passed as parameters to functions. Thus, we can further generalize the example with the following function sigature

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

### Other Slices

For example, we can slide an array:

```rust
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
```
