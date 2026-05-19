// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    Red,
    Green,
    Blue,
}

fn print_color(c: Color) {
    let name = match c {
        Color::Red => "Red",
        Color::Green => "Green",
        Color::Blue => "Blue",
    };
    println!("{:?}", name);
}

fn main() {
    print_color(Color::Red);
    print_color(Color::Green);
    print_color(Color::Blue);
}
