# Chapter 11: Writing Automated Tests

**Table of Contents**
- [11.1 - How to Write Tests](#111---how-to-write-tests)
- [11.2 - Controlling How Tests Are Run](#112---controlling-how-tests-are-run)
- [11.3 - Test Organization](#113---test-organization)

## 11.1 - How to Write Tests

The bodies of test functions typically perform these three actions:

1. Set up any needed data or state
1. Run the code you want to test
1. Assert that the results are what you expect

### The Anatomy of a Test Function

Simply, a test in Rust is a function that's annotated with the `#[test]` attribute. You can run tests with the `cargo test` command.

See examples in [*adder/*](adder/).

### Checking Results with the `assert!` Macro

We can use `assert!` macro to check if a condition is true, when a method or function returns a Boolean value.

### Testing Equality wit the `assert_eq!` and `assert_ne!` Macros

These macros check that two values are equal or not equal, respectively. Values being compared must implement the `PartialEq` and `Debug` traits.

See examples in [*adder/*](adder/).

### Adding Custom Failure Messages

We can add custom failure messages to aforementioned macros to provide more information about what went wrong when a test fails. The argument location for the custom failure messages follows directly from standard arguments.

See examples in [*adder/*](adder/).

### Checking for Panics with `should_panic`

We can write tests that check whether a function panics when it should. This is useful for testing error handling code.

See examples in [*adder/*](adder/).

### Checking for Panics with `should_panic`

We can write tests that check whether a function panics when it should. This is useful for testing error handling code.

See examples in [*adder/*](adder/).

### Using `Result<T, E>` in Tests

Instead of using `assert!`, `assert_eq!`, or `assert_ne!` macros, we can use `Result<T, E>` to handle errors. Benefits of using `Result<T, E>` include:

- **Readability**: `Result<T, E>` makes it clear that a function can fail and provides a way to handle the error.
- **Flexibility**: `Result<T, E>` allows for more complex error handling, such as chaining errors or returning a custom error type.
- **Type Safety**: `Result<T, E>` ensures that the function's return type is consistent and predictable.

It enables you to use the `?` operator to propagate errors and handle them in a consistent way.
