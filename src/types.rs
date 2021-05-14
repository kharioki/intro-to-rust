/**
Primitive Types--
Integers:  u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory) - signed and unsigned(non-negative)
Floats: f32, f64
Boolean (bool) - the convention for booleans is snake case
Characters (char)
Tuples
Arrays - fixed length, vectors - growable length
*/

// Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.
// meaning you don't have to set the type for every single variable you create

pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 4567890;

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // get boolean from expression
    let is_greater: bool = 10 > 15;

    // characters - char
    let a1 = 'a';

    // emoji - unicode
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}