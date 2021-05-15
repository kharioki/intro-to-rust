// Arrays - Fixed list where elements are the same data types
// if an array is mutable we can't add on to it but we can reassign values
use std::mem;

pub fn run() {
    // basic array
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // re-assign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // get single array value
    println!("val: {}", numbers[0]);

    // get array length
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    // get array memory
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // get slices from array
    let slice: &[i32] = &numbers[0..2];

    println!("Slice: {:?}", slice);
}