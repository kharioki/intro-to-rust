pub fn run() {
    // print to console
    println!("Hellow from the print.rs file");

    // formatting integers
    println!("Number: {}", 1);

    // formatting strings
    println!("{} is from {}", "Tony", "Endarasha");

    // positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Tony", "Endarasha", "write code");

    // Named arguments
    println!("{name} like to play the {activity}", name = "Tessie", activity = "drums");

    // placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug traits
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}