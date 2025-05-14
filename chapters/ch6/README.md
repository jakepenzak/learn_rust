# Chapter 6: Enums and Pattern Matching

*Enumerations* (or *enums*) allow you to define a type by enumerating its possible *variants*.

**Table of Contents**
- [6.1 - Defining an Enum](#61---defining-an-enum)
- [6.2 - The `match` Control Flow Construct](#62---the-match-control-flow-construct)
- [6.3 - Concise Control Flow with `if let` and `let else`](#63---concise-control-flow-with-if-let-and-let-else)


## 6.1 - Defining an Enum

*Enums* give a way of saying a value is one of a possible set of values. For example, a `Rectangle` is one of a set of possible shapes that also includes `Circle` and `Triangle`. See [`ch5/rectangles`](../ch5/rectangles) for a toy example with shapes.

IP Address example:

```rust
enum IpAddrKind {
    V4,
    V6
}
```

### Enum Values

To create instances:

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

We can now use this type to define a function that takes an `IpAddrKind` as an argument:

```rust
fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("Routing for IPv4"),
        IpAddrKind::V6 => println!("Routing for IPv6"),
    }
}

route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

To use this, we could add this type to a `struct`, but we can also do it directly with `enums`:

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"))
let loopback = IpAddr::V6(String::from("::1"))

// We could also do

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

We automatically get a constructor function defined as a result of defining the enum.

This is particularly useful over structs if we want to define a function that takes a common type as an argument.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// We could do this with structs
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

// But writing a function that takes type Message is easier

fn is_instance_of_message(message : &Message) -> bool {
    // ...
}
```

We can also add methods to enums!

```rust

impl Message {
    fn call(&self) {
        // ...
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

### The `Option` Enum and Its Advantages Over Null Values

```rust
enum Option<T> {
    None,
    Some(T),
}
```

Option is an enum that represents an optional value. It is used to handle the absence of a value in a way that is safe and expressive. `<T>` represents the type of the value that may or may not be present, allowing `Option` to be of different types dynamically.

```rust
let some_number = Some(5);
let absent_number: Option<i32> = None;

// Below won't work
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

This second portion fails b/c they are different types. In other words, you have to convert an `Option<T>` to a `T` before you can perform `T` operations with it. This helps ensure that your code is safe and free from null pointer dereferences. It forces you to explicitely state when something can be null and handle that case directly. How do we actually handle these cases explicitely? With `match`!

## 6.2 - The `match` Control Flow Construct

The `match` control flow construct is a powerful tool for pattern matching in Rust. It allows you to match a value against a set of patterns and execute code based on the match. Key difference with `if` is that `if` must return a boolean, whereas `match` can return any type.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin:Dime => 10,
        Coin:Quarter => 25,
    }
}
```

### Patterns That Bind to Values

Patterns can also bind to values in the pattern itself. This is useful when you want to extract values from an enum.

```rust
#[derive(Debug)] // to inspect
enum UsState {
    Alabama,
    Alaska,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
```

### Matching with `Option<T>`

Following from section 6.1, we can use `match` to handle `Option<T>` patterns:

```rust
fn plus_one(x:Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

### Matches are Exhaustive

With `match` the arms' patterns must cover all possibilities of the type (e.g., we need to handle all cases explicitly).

### Catch-all Patterns and the `_` Placeholder

We can use `other` to catch all other cases, binding to the value of those cases, or `_` to handle other cases, without bind to the value.

```rust
// Using other
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

// Using _
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(), // or () for empty tuple
}
fn reroll() {}
```

## 6.3 - Concise Control Flow with `if let` and `let else`

The `if let` syntax lets you combine `if` and `let` into a less verbose way to handles values that match one pattern while ignoring the rest.

```rust
// Verbose case
let config_max = Some(3u8); // 3u8 is the same as let x: u8 = 3
match config_max {
    Some(max) => println!("The maximum is configured to be {}", max),
    _ => (),
}
// Concise case
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}
```

This is useful, but you do lose exhaustive checking of `match`. Ultimately, `if let` is syntax sugar for a `match` that runs code when the value matches one pattern and ignores the rest.

You can include `else` with an `if let` as well:

```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {state:?}!");
} else {
    count += 1
}

// This is equivalent to
match coin {
    Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    _ => count += 1,
}
```

### Staying on the "happy path" with `let else`

Consider the following example:

```rust
impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            // ...
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

// We can handle this more succinctly with let else

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
```

> [!NOTE]
> In general, use `match` and when it is starting to get overly verbose, remember `if let` and `let else` exist in the toolbox!
