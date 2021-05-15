// Tuples group together values of different values
// Max 12 elements

pub fn run() {
    // basic tuple
    let person: (&str, &str, i8) = ("Tony", "Westlands", 29);

    println!("{} is from {} and is {} years old.", person.0, person.1, person.2);
}