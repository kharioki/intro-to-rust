# intro-to-rust

### Contents
- [intro-to-rust](#intro-to-rust)
    - [Contents](#contents)
    - [What is Rust?](#what-is-rust)
      - [What is Cargo?](#what-is-cargo)
        - [Starting a new project](#starting-a-new-project)
        - [Running a project](#running-a-project)
        - [Adding dependencies](#adding-dependencies)
        - [Adding dev dependencies](#adding-dev-dependencies)
        - [Running tests](#running-tests)
        - [Running benchmarks](#running-benchmarks)
      - [What is a crate?](#what-is-a-crate)
  - [Rust Fundamentals](#rust-fundamentals)
    - [Variables](#variables)
      - [Defining Constants](#defining-constants)
      - [Scope and Shadowing](#scope-and-shadowing)
      - [Memory Safety](#memory-safety)
    - [Functions](#functions)
    - [Module System](#module-system)
    - [Primitive Types](#primitive-types)
      - [Scalar Types](#scalar-types)
      - [Compound Types](#compound-types)
    - [Control Flow](#control-flow)
      - [`if` Expressions](#if-expressions)
      - [Expressions with `else if`](#expressions-with-else-if)
      - [Simplifying `if` Expressions](#simplifying-if-expressions)
      - [`loop` Expressions](#loop-expressions)


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

### Primitive Types

#### Scalar Types

There are four primary scalar types in Rust: _integers_, _floating-point numbers_, _Booleans_, and _characters_.

**Integers** are numbers without a fractional component. Rust has two integer types: _signed_ and _unsigned_. Signed integers store numbers with a negative or positive sign. Unsigned integers store only positive numbers. The `i8` type is an 8-bit signed integer, and the `u8` type is an 8-bit unsigned integer. The default integer type is `i32`. The `isize` and `usize` types depend on the kind of computer your program is running on: 64 bits if you're on a 64-bit architecture and 32 bits if you're on a 32-bit architecture.
integer literals can be written as decimal (98_222), hexadecimal (0xff), octal (0o77), or binary (0b1111_0000), and as unsuffixed integer or as suffixed integer by the type.

  | literal      | Example |
  | :------------: | :-----------: |
  | Decimal      | 98_222       |
  | Hexadecimal   | 0xff        |
  | Octal   | 0o77        |
  | Binary   | 0b1111_0000        |
  | Byte (u8 only)   | b'A'        |


**Floating-Point Types** are numbers with a fractional component. Rust has two primitive types for floating-point numbers: `f32` and `f64`. The default type is `f64` because on modern CPUs it's roughly the same speed as `f32` but is capable of more precision. Floating point numbers follow the IEEE-754 standard. You do always need to have atleast one digit before and after the decimal point.

you can suffix integers and floating-point numbers with the type of number as well: `5u16` or `5_u16`, `57.0f64` or `57f64`.

**Boolean Type** is a type that has only two possible values: `true` and `false`. Booleans are one byte in size. The boolean type is specified using the `bool` keyword.

**Character Type** is the most primitive alphabetic type, and the `char` type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Unicode Scalar Values range from `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF` inclusive. Rust's `char` type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Unicode Scalar Values range from `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF` inclusive. Accented letters, Chinese/Japanese/Korean characters, emoji, and zero-width spaces are all valid `char` values in Rust. The `char` type is specified with single quotes, as opposed to the double quotes used for `String` values.

#### Compound Types

Compound types can group multiple values into one type. Rust has two primitive compound types: _tuples_ and _arrays_.

**Tuples** are a general way of grouping together a number of values with a variety of types into one compound type. 
- Tuples have a fixed length: once declared, they cannot grow or shrink in size. 
- Tuples are useful when you want to return multiple values from a function. 
- Tuples are created using parentheses `()`. 
- Each position in the tuple has a type, and the types of the different values in the tuple don't have to be the same. Tuples can be destructured to create bindings to their individual pieces.

```rust
fn main() {
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (x, y, z) = tup;
  println!("The value of y is: {}", y);
}
```

There are two ways to access members of a tuple: destructuring and dot notation. 
- _Destructuring_ is a convenient way to assign a tuple's values to variables. 
- _Dot notation_ is a convenient way to access a tuple's individual values.

```rust
fn main() {
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let five_hundred = tup.0;
  let six_point_four = tup.1;
  let one = tup.2;

  // destructuring
  let (x, y, z) = tup;
  println!("The value of y is: {}", y);
}
```

_A tuple has a maximum **arity** of 12. Meaning, a tuple can have a maximum of 12 elements_

**Arrays** are similar to tuples, but every element of an array must have the same type. Arrays in Rust are different from arrays in some other languages because arrays in Rust have a fixed length, like tuples.

Arrays are useful when you want your data allocated on the stack rather than the heap or when you want to ensure you always have a fixed number of elements. 

Arrays are created using square brackets `[]`. The type of the array and the length of the array are both required. The length of the array is part of its type. 

```rust
fn main() {
  let a = [1, 2, 3, 4, 5];
  let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

  let a: [i32; 5] = [1, 2, 3, 4, 5]; // here the type is i32 and the length is 5
  let a = [3; 5];
}
```

You can also initialize an array to contain the same value for each element by specifying initial value followed by a semicolon and then the length of the array in square brackets.

```rust
fn main() {
  let a = [3; 5];
  println!("{:?}", a); // [3, 3, 3, 3, 3]
}
```

You can access an array element by indexing the array. The index of an array starts at 0. 

```rust
fn main() {
  let a = [1, 2, 3, 4, 5];
  let first = a[0];
  let second = a[1];
}
```

> **Note:** If you try to access an element using an index that is greater than or equal to the length of the array, your program will panic and exit. Rust will check that the index you've specified is less than the array length at runtime and panic if the index is out of bounds. This is an example of Rust's memory safety feature.

> **Note:** Arrays are limited to a size of 32 elements. If you need to work with a larger set of data, use a vector instead.

### Control Flow

Control flow refers to the ability to run some code depending on some condition. Rust has three keywords for control flow: `if`, `else`, and `loop`.

#### `if` Expressions

An `if` expression allows you to run some code if a condition is true and run some other code if the condition is false. 

```rust 
fn main() {
  let number = 3;

  if number < 5 {
    println!("condition was true");
  } else {
    println!("condition was false");
  }
}
```

You can also use `if` in a `let` statement. 

```rust
fn main() {
  let condition = true;
  let number = if condition { 5 } else { 6 };

  println!("The value of number is: {}", number);
}
```

> **Note:** The condition must be a `bool`. If it isn't, you'll get an error.

#### Expressions with `else if`
You can handle multiple conditions using `else if`.

```rust
fn main() {
  let number = 6;

  if number % 4 == 0 {
    println!("number is divisible by 4");
  } else if number % 3 == 0 {
    println!("number is divisible by 3");
  } else if number % 2 == 0 {
    println!("number is divisible by 2");
  } else {
    println!("number is not divisible by 4, 3, or 2");
  }
}
```

#### Simplifying `if` Expressions

> **Note:** - If is an expression not a statement, an expression returns a value, a statement does not. Meaning you can use it on the right side of a `let` statement.

```rust
let msg = if num == 5 { // msg is assigned the value of the expression
  "five"  // no semicolon here -> tail expression
} else if num == 6 {
  "six"  // we cant use return -> return only allowed in functions
} else {
  "not five or six" // all return values must have the same type
}; // semicolon here -> statement
```

One line `if` `else` expressions

```rust
num = if condition { 5 } else { 6 };
```

#### `loop` Expressions

A `loop` expression allows you to loop forever or until you explicitly tell it to stop. Rust has three kinds of loops: `loop`, `while`, and `for`.

```rust
fn main() {
  loop {
    println!("again!");
  }
}
```

A loop will continue to execute until you explicitly tell it to stop. You can do this by using the `break` keyword.

```rust
fn main() {
  let mut counter = 0;

  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2;
    }
  };

  println!("The result is {}", result);
}
```

