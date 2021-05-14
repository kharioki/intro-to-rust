// Variables hold primitive data or references to data
// Variables are immutable by default - you can't reassign them
// Rust is a blocked-scope language - if you set a variable to a function, it pertains to that scope

pub fn run() {
    // assigning variables
    let name = "Tony Stark";

    // mutable variables
    let mut age = 29;

    println!("My name is {} and I am {}", name, age);

    // mutating variable
    age = 30;

    println!("My name is {} and I am {}", name, age);

    // Define constants - const keyword
    // should be all uppercase
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let ( my_name, my_age ) = ("Kharioki", 29);
    println!("{} is {}", my_name, my_age);
}