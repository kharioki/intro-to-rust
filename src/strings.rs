/**
There are two types of strings
    Primitive str = Immutable fixed-length string somewhere in memory
    String = Growable, heap-allocated data structure - used when you need to modify your own string data
*/

pub fn run() {
    // basic str
    let hello = "hello";

    // Sring
    let mut wassup = String::from("Wassup! ");

    // get length - works for either primitive str or String
    println!("Length: {}", hello.len());
    println!("Length: {}", wassup.len());

    // push - add only one char at a time
    wassup.push('T');

    // push_str - push more than one char (a string)
    wassup.push_str(", how are you?");

    /*
     string methods
    */

    // Capacity in bytes
    println!("Capacity: {}", wassup.capacity());

    // is_empty
    println!("Is Empty: {}", wassup.is_empty());

    // contains 
    println!("Contains 'how': {}", wassup.contains("how"));

    // replace
    println!("Replace: {}", wassup.replace("T", "Tony"));

    // loop through sring by whitespace
    for word in wassup.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    // Assertion testing - left === right
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());


    println!("{:?}", (hello, wassup));
}