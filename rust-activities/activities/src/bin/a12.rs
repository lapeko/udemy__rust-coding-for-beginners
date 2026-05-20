// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
use std::fmt;

enum Color {
    Yellow,
    Green,
}

struct Box {
    dimensions: i32,
    weight: f32,
    color: Color,
}

impl Box {
    fn new(dimensions: i32, weight: f32, color: Color) -> Self {
        Self {
            dimensions: dimensions,
            weight: weight,
            color: color,
        }
    }

    fn print_characteristics(&self) {
        println!("{}", self);
    }
}

impl fmt::Display for Box {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Box: {{
    dimensions: {:?},
    weight: {:?},
    color: {:?},
}}",
            self.dimensions,
            self.weight,
            match self.color {
                Color::Yellow => "yellow",
                Color::Green => "Green",
            }
        )
    }
}

fn main() {
    Box::new(3, 120.5, Color::Green).print_characteristics();
}
