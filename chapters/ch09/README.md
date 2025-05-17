# Chapter 9: Error Handling

Two main types of errors in Rust: unrecoverable and recoverable.

Unrecoverable errors are those that cannot be handled by the program (e.g., *division by zero*). When an unrecoverable error occurs, the program should panic and terminate.

Recoverable errors are those that can be handled by the program (e.g., *file not found*). When a recoverable error occurs, the program should return an error value that can be handled by the caller.

Rust doesn't have exceptions. It has `Result<T, E>` for recoverable errors and `panic!` for unrecoverable errors.

**Table of Contents**
- [9.1 - Unrecoverable Errors with `panic!`](#91---unrecoverable-errors-with-panic)
- [9.2 - Recoverable Errors with `Result`](#92---recoverable-errors-with-result)
- [9.3 - To `panic!` or Not to `panic!`](#93---to-panic-or-not-to-panic)

## 9.1 - Unrecoverable Errors with `panic!`

Two ways to cause a panic in practice: by taking an action that causes our code to panic or by explicitly calling `panic!`.

```rust
fn main() {
    panic!("cash and burn");
}
```

To return backtrace, we can run with `RUST_BACKTRACE` env variable set (e.g., `RUST_BACKTRACE=1 cargo run`).


## 9.2 - Recoverable Errors with `Result`

In many cases, errors aren't that serious and we can take specific action based on the error using `Result` enum:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E)
}
```

We can match based on `Result`:

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}
```

### Matching on different errors

In the above example, we fail for all errors, but suppose we want to create file if it doesn't exist, but fail for other reasons (e.g., permissions):

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {e:?}"),
        },
        _ => {
            panic!("Problem opening the file: {error:?}");
        }
    },
    };
}
```

Cleaner approach (see [ch. 13](../ch13/)):

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}
```

#### Shortcuts for Panic on Error: `unwrap` and `expect`

`unwrap` is a shortcut for `match` that panics if the value is `Err` else return the value:

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
```

`expect` is a shortcut for `match` that panics with a custom message if the value is `Err`:

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

Generally, you should always use `expect` to provide more detail.

### Propagating Errors

Instead of handling errors directly in the function itself, you can return the error to the calling code to allow the caller to decide how to handle the error.

```rust
use std::fs::File;
use std::io::{self,Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}
```

#### A Shortcut for Propagating Errors: the `?` Operator

The same functionality of above can be constructed via:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?; // `?` propagates any errors here
    let mut username = String::new();
    username_file.read_to_string(&mut username)?; // `?` propagates any errors here
    Ok(username)
}
```

Key difference from above is the `?` using `from` function under the hood which converts all possible errors to one defined in function signature!

We could further reduce verbosity via:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_string(&mut username)?; // chaining commands here

    Ok(username)
}
```

And, finally, standard library provides this out of the box:

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

#### Where the `?` operator can be used

`?` must be used in functions that return `Result`, `Option`, or `impl FromResidual` types or handled explicitely in the code via, say, `match`.

With `Option` instead of returning the error as in `Result` it will return `None` early.

```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

You can combine both `Result` and `Option` in a function without using methods like `ok` on `Result` or `ok_or` on `Option`.

You can modify the `main` function to return `Result` type.

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> { // Box<dyn Error> allows for any error (See ch.18)
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
```

Main function can also return `Termination` trait. See chapter for more details.

## 9.3 - To `panic!` or Not to `panic!`

In general, `Result` is a good default choice when you're defining a function that may fail in some cases. Leave it up to the caller on how to handle the situation!

For situations such as examples, prototype code, and tests, it's more appropriate to panic.

See [chapter](https://doc.rust-lang.org/stable/book/ch09-03-to-panic-or-not-to-panic.html) for more details and general guidelines.
