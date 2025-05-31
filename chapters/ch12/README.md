# Chapter 12: An I/O Project: Building a Command Line Program

We'll create our own version of `grep` command.

This project will combine techniques learned thus far, including:
- [Organizing code](../ch07)
- [Using vectors and strings](../ch08)
- [Handling errors](../ch09)
- [Using traits and lifetimes where appropriate](../ch10)
- [Writing tests](../ch11)

**Table of Contents**
- [12.1 - Accepting Command Line Arguments](#121---accepting-command-line-arguments)
- [12.2 - Reading a File](#122---reading-a-file)
- [12.3 - Refactoring to Improve Modularity and Error Handling](#123---refactoring-to-improve-modularity-and-error-handling)
- [12.4 - Developing the Library's Functionality with Test-Driven Development](#124---developing-the-librarys-functionality-with-test-driven-development)
- [12.5 - Working with Environment Variables](#125---working-with-environment-variables)
- [12.6 - Writing Errors Messages to Standard Error Instead of Standard Output](#126---writing-errors-messages-to-standard-error-instead-of-standard-output)

## 12.1 - Accepting Command Line Arguments

First, we'll want to be able to read arguments to CLI tool. To do this,
we can leverage `std::env::args` function provided by standard library, which returns
command line arguments as an iterable (covered more in [ch13](../ch13))

## 12.2 - Reading a File

We can read in files using `std::fs::read_to_string` function provided by standard library, which returns
the contents of a file as a string.

## 12.3 - Refactoring to Improve Modularity and Error Handling

**Separation of Concerns for Binary Projects**

The organizational problem of allocating responsibility for multiple tasks to the main function is common to many binary projects. As a result, the Rust community has developed guidelines for splitting the separate concerns of a binary program when main starts getting large. This process has the following steps:
- Split your program into a main.rs file and a lib.rs file and move your program’s logic to lib.rs.
- As long as your command line parsing logic is small, it can remain in main.rs.
- When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.

The responsibilities that remain in the main function after this process should be limited to the following:
- Calling the command line parsing logic with the argument values
- Setting up any other configuration
- Calling a run function in lib.rs
- Handling the error if run returns an error

## 12.4 - Developing the Library's Functionality with Test-Driven Development


In this section, we’ll add the searching logic to the minigrep program using the test-driven development (TDD) process with the following steps:

1. Write a test that fails and run it to make sure it fails for the reason you expect.
2. Write or modify just enough code to make the new test pass.
3. Refactor the code you just added or changed and make sure the tests continue to pass.
4. Repeat from step 1!

## 12.5 - Working with Environment Variables

We can use and read environment variables in Rust using the `std::env` module. For example, we can read the value of the `IGNORE_CASE` environment variable to determine whether to ignore case when searching for a query in a file.

## 12.6 - Writing Errors Messages to Standard Error Instead of Standard Output

By default, Rust programs write errors to standard output. However, it is more common to write errors to standard error. This is because standard output is typically used for program output, while standard error is typically used for error messages.

We can print to standard error via the `eprintln!` macro. For example, we can print an error message to standard error using the following code:

```rust
eprintln!("An error occurred: {}", error);
```
