/*  
  - Variables have a scope - the place in the code that you are allowed to use them.
  - Rust has a number of rules that govern scope, but the most important rule is that
    each scope is a nested set of curly braces.
  - It begins with an opening curly brace and ends with a closing curly brace.
  - When a variable goes out of scope, it is dropped.
  - The drop function is called automatically at the closing curly brace.
  - The drop function is where the author of String can put the code to return the memory.
 */


pub fn main() {
  let x = 5;
  {
    let y = 10;
    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x); // x is in scope here
  }
  println!("The value of x is: {}", x); // x is in scope here
  // println!("The value of y is: {}", y); // y is out of scope here so Error
}

fn new_mut() {
  let mut x = 5; // x is mutable
  let x = x + 1; // x is immutable
}

pub fn run() {
    let a = String::from("hello"); // a comes into scope

    takes_ownership(a); // a's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
} // Here, x goes out of scope, then a. But because a's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
