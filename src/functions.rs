// Functions - Used to store blocks of code for re-use

pub fn run() {
    greeting("Wassup", "Kharioki");

    // Bind function values to variables
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // closure - can also use outside variables
    let n3: i32 = 15;
    let add_nums =  |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C sum: {}", add_nums(12,13));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}