# intro-to-rust

### What is Rust?
Rust is an extremely fast and powerful **systems programming language**. 

It is a systems language like C, C++, Golang; built more for systems than application programming languages like Java, Javascript, C#.

Rust pursues the tri-fecta: `safety`, `speed`, and `concurrency`.

> Languages like `Ruby` and `Python` will give you `speed` and `concurrency` but not `safety`. Languages like `C` and `C++` will give you `safety` and `speed` but not `concurrency`. Rust gives you all three.

#### What is Cargo?

Cargo is Rust's package manager and build tool. It is similar to `npm` or `yarn` in Javascript.

##### Starting a new project

To start a new project, run `cargo new <project-name>`. This will create a new directory with the name of your project and a `Cargo.toml` file.

##### Running a project

To run a project, run `cargo run`. This will compile your project and run it.
  - If you want to compile your project without running it, run `cargo build`. This will create a `target` directory with a `debug` directory inside of it. Inside of the `debug` directory, you will find your compiled project.

##### Adding dependencies

To add a dependency, add the dependency to the `Cargo.toml` file. For example, to add the `rand` crate, add `rand = "0.7.3"` to the `[dependencies]` section of the `Cargo.toml` file.

##### Adding dev dependencies

To add a dev dependency, add the dependency to the `Cargo.toml` file. For example, to add the `rand` crate, add `rand = "0.7.3"` to the `[dev-dependencies]` section of the `Cargo.toml` file.

##### Running tests

To run tests, run `cargo test`. This will run all tests in the `tests` directory.

##### Running benchmarks

To run benchmarks, run `cargo bench`. This will run all benchmarks in the `benches` directory.

#### What is a crate?

A crate is a package of Rust code. It is similar to a `package.json` file in Javascript.

## Rust Fundamentals

### Variables

They hold primitive data or references to data.
- Variables are immutable by default -> meaning you can't reassign them.
- Rust is a block-scoped language -> meaning variables are only accessible within the block they are declared in.
- we use the `let` keyword to declare variables -> `let <variable-name> = <value>;`
- we add the `mut` keyword to make a variable mutable -> `let mut <variable-name> = <value>;`
- we can define multiple variables in one line -> `let (<variable-name-1>, <variable-name-2>) = (<value-1>, <value-2>);`

> With immutability, _**Safety**_ is enforced by the compiler. With mutability, Safety is enforced by the programmer.Data that never changes can be shared between multiple threads so _**Concurrency**_ is improved and compiler can do extra optimizations to make the code run faster hence _**Speed**_ is improved.

#### Defining Constants

Constants are always immutable and are always in scope.
- we use the `const` keyword to declare constants -> `const <constant-name>: <type> = <value>;`
- should be declared in all caps -> `const MAX_POINTS: u32 = 100_000;`
- type annotation is required -> `const MAX_POINTS = 100_000;` will throw an error
- value must be constant -> `const MAX_POINTS: u32 = get_max_points();` will throw an error. Value must be known at compile time.

> Constants are useful when you want to make sure a certain value is always the same throughout your entire program. They can be declared in any scope, including the global scope.

#### Scope and Shadowing

Variables have a scope - the place in the code that you are allowed to use them.
  - Rust has a number of rules that govern scope, but the most important rule is that
    each scope is a nested set of curly braces.
  - It begins with an opening curly brace and ends with a closing curly brace.
  - When a variable goes out of scope, it is dropped.
  - The drop function is called automatically at the closing curly brace.
  - The drop function is where the author of String can put the code to return the memory.

Shadowing is a way to declare a new variable with the same name as a previous variable.

#### Memory Safety

Rust guarantees memory safety at compile time. This is done by the compiler checking the code for you. If the code doesn't pass the checks, the compiler will throw an error.

Variables must be initialized before use. 

_Example:_
```rust
fn main() {
  let enigma: i32;
  println!("{}", enigma); // will throw an error -> use of possibly uninitialized variable: `enigma`
}

fn new_main() {
  let enigma: i32;
  if true {
    enigma = 42; // initializing enigma depending on some condition
  }
  println!("{}", enigma); // Error -> the compiler won't reason about the value of a condition at compile time even if it's a literal true or false. 
  // Conditional evaluation is handled at runtime.
  // Compiler can't guarantee that the condition will be true at runtime.
}

fn new_new_main() {
  let enigma: i32;
  if true {
    enigma = 42; // initializing enigma depending on some condition
  } else {
    enigma = 0;
  }
  println!("{}", enigma); // this works because the compiler can reason about the value of the condition at compile time.
}
```

___
### Functions

Functions are the primary way to organize code in Rust.
- Functions are declared using the `fn` keyword -> `fn <function-name>(<parameter-name>: <type>) -> <return-type> { <function-body> }`
- the rust guide recommends using `snake_case` for function names -> `fn my_function() { ... }`
- functions don't have to appear in the file before they are called -> Rust will compile the code even if the function is called before it is defined.
- function parameters are always defined with a type -> `fn my_function(<parameter-name>: <type>) { ... }`
- you specify the return type of a function by using the `->` operator -> `fn my_function() -> <return-type> { ... }`
- the body of a function is a block of code -> `{ ... }`
- you can return a value from a function by using the `return` keyword -> `return <value>;`
- there's a shorthand for returning a value from a function. If the last expression in a function is an expression, the function will return the value of that expression -> `fn my_function() -> <return-type> { <expression> }`
- **Tail expressions** are expressions that are evaluated last in a function. The value of a tail expression is returned from the function.
```rust
  { return true; } // this is not a tail expression
  { true } // this is a tail expression
```
- there's currently no support for named arguments at the call site, so you need to provide arguments in the same order as the parameters are defined in the function signature.

```rust
fn main() {
  let x = do_stuff(2.0, 12.5);
}

fn do_stuff(x: f64, y: f64) -> f64 {
  x + y
}
```

- A simple rust function does not support variable number of arguments. You can use a vector to pass a variable number of arguments to a function.

```rust
fn main() {
  let x = do_stuff(2.0, 12.5, 3.0, 4.0, 5.0);
}

fn do_stuff(x: f64, y: f64, args: Vec<f64>) -> f64 {
  let mut sum = x + y;
  for arg in args {
    sum += arg;
  }
  sum
}
```

- **What are macros?** Macros are a way to define code that generates other code. They are similar to functions, but instead of generating a new function, they generate new code that gets compiled with the rest of the code.
- Macros are defined using names that end with an exclamation mark -> `macro_rules! <macro-name> { ... }`

___

### Module System

Modules are a way to organize code into groups of related functionality. Modules also control the privacy of items, which is whether an item can be used by outside code (public) or is an internal implementation detail and not available for outside use (private).

As we write large programs, organizing our code becomes increasingly important. By grouping related functionality and separating code with distinct features, you'll clarify where to find code that implements a particular feature and where to go to change how a feature works. 

As a project grows, you should organize code by splitting into multiple modules, then multiple files. 

Rust has a number of features that allow you to manage your code's organization, including which details are exposed, which detaills are private, and how to bring paths into scope. These include:
- **packages** - a Cargo feature that lets you build, test, and share crates
- **crates** - a tree of modules that produces a library or executable
- **modules** and **use** - let you control the organization, scope, and privacy of paths
- **paths** - a way of naming an item, such as a struct, function, or module

A _package_ is one or more crates that provide a set of functionality. A _crate_ is a binary or library. A binary is an executable, which is produced from a crate's source code. A library is code that's intended to be used with other code.

A crate can come in one or two forms: a _binary crate_ or a _library crate_. A binary crate is an executable, which is produced from a crate's source code. A library crate is code that's intended to be used with other code and does not have a main function.