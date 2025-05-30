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

## 11.2 - Controlling How Tests Are Run

`cargo test` arguments can go to either (a) `cargo test` or (b) the resulting test binary. To see options for (a) run `cargo test --help` or (b) run `cargo test -- --help` for options specific to the test binary.

### Running Tests in Parallel or Consecutively

Running tests in parallel or consecutively can be controlled using the `--test-threads` flag. By default, tests are run in parallel using all available CPU cores. To run tests consecutively, use `--test-threads=1`.

### Showing Function Output

By default, when a test passes, Rust captures anything printed to standard output, unless it fails. Then it prints standard output and standard error.

To show the ouput of successful tests as well, we can run `cargo test -- --show-output`.

### Running a Subset of Tests by Name

By default, `cargo test` runs all tests.

To run a single test, we can run `cargo test <test_function_name>`.

To run multiple tests using filtering, we can run `cargo test <pattern>`. Note that the module in which a test appears becomes part of it's name so we can use that in the pattern.

### Ignoring Some Tests unless Specifically Requested

To ignore certain tests, we can use the `#[ignore]` attribute. This will cause the test to be skipped when running `cargo test`. To run only ignored tests, we can use the `-- --ignored` flag. To run all tests, including ignored, we can use the `-- --include-ignored` flag.

## 11.3 - Test Organization

The Rust community thinks about tests in terms of two main categories:

1. *Unit Tests* - small and more focused, testing one module in isolation at a time, and can test private interfaces.
  - Put unit tests in the *src* directory in each file with the code they're testing. The convention is to create a module named `tests` in each file to contain the test functions and to annotate the module with `cfg(test)`
  - `#[cfg(test)]` tells Rust to compile and run the test code only when you run `cargo test`.

```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

2. *Integration Tests* - entirely external to your library and use your code in the same way any other external code would, using only the public interaface and potential exercising multiple modules per test.
  - For integration tests, we'd create a *tests* directory at the project directory next to *src*
  - To run integration tests, we can still use `cargo test` and it will run once all unit tests complete
  - To run only specific integration tests, we can use `cargo test --test integration_test`
  - We can use *tests/common/* directory for shared components.
