// Vectors - resizable arrays

use std::mem;

pub fn run() {
    // basic array
    let mut numbers: Vec<i32> = vec![6, 7, 8, 9];

    // re-assign value
    numbers[3] = 10;

    // Add on to vectors - push
    numbers.push(11);
    numbers.push(12);

    // pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    // get single array value
    println!("val: {}", numbers[0]);

    // get vector length
    println!("Vector length: {}", numbers.len());

    // Vectors are stack allocated
    // get array memory
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // get slices from array
    let slice: &[i32] = &numbers[0..2];

    println!("Slice: {:?}", slice);

    // loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}