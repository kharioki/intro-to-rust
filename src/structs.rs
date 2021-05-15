// Structs - Used to create custom data types

// Traditional struct - conventionally we name structs in Uppercase
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple struct
struct Colour(u8, u8, u8);

// functional structs
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct a person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string()
    }

    // name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);
    
    let mut cc = Colour(250, 0, 0);

    cc.1 = 5;

    println!("Colour: {} {} {}", cc.0, cc.1, cc.2);

    let mut p = Person::new("John", "Doe");

    println!("Person: {} {}", p.first_name, p.last_name);
    println!("Full name: {}", p.full_name());
    println!("Person tuple: {:?}", p.to_tuple());
    
    let mut w  = Person::new("Mary", "Doe");
    println!("Wife's original name: {}", w.full_name());

    w.set_last_name("Williams");

    println!("Wife's married name: {}", w.full_name());

}