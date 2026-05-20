// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

use std::fmt;

enum Color {
    Red,
    Blue,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::Red => "Red",
                Color::Blue => "Blue",
            }
        )
    }
}

struct Person {
    age: i32,
    name: String,
    color: Color,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Person {{
    age: {},
    name: {},
    color: {},
}}",
            self.age, self.name, self.color,
        )
    }
}

fn main() {
    let v = vec![
        Person {
            age: 23,
            name: "Stephan Kapusta".to_owned(),
            color: Color::Blue,
        },
        Person {
            age: 7,
            name: "Crazy relax".to_owned(),
            color: Color::Red,
        },
        Person {
            age: 45,
            name: "Lost life coach".to_owned(),
            color: Color::Blue,
        },
    ];

    for p in v {
        if p.age <= 10 {
            println!("{}", p);
        }
    }
}
