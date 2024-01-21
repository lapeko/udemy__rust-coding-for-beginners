enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let color = Color::Red;

    print_color(color);
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
}
