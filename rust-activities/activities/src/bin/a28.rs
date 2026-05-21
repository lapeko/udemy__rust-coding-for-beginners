// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

#[derive(Debug, Clone)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

#[derive(Debug)]
struct ShirtColor(Color);

impl ShirtColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

#[derive(Debug)]
struct PantsColor(Color);

impl PantsColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

#[derive(Debug)]
struct ShoesColor(Color);

impl ShoesColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

fn display_shirt(shirt: ShirtColor) {
    println!("Selected shirt color: {:?}", shirt.0);
}

fn display_pants(pants: PantsColor) {
    println!("Selected pants color: {:?}", pants.0);
}

fn display_shoes(shoes: ShoesColor) {
    println!("Selected shoes color: {:?}", shoes.0);
}

fn main() {
    let my_shirt = ShirtColor::new(Color::Blue);
    let my_pants = PantsColor::new(Color::Black);
    let my_shoes = ShoesColor::new(Color::Custom("Red with neon stripes".to_owned()));

    display_shirt(my_shirt);
    display_pants(my_pants);
    display_shoes(my_shoes);
}
