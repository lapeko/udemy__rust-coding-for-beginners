fn main() {
    let sh_box = ShippingBox::create_new(20, 10, 30, 10, Color::Blue);
    sh_box.print();
}

struct Dimensions {
    width: i32,
    height: i32,
    depth: i32,
}

impl Dimensions {
    fn print(&self) {
        println!("width: {}", self.width);
        println!("height: {}", self.height);
        println!("depth: {}", self.depth);
    }
}

enum Color {
    Red,
    Blue,
}

impl Color {
    fn print(&self) {
        println!("color: {}", match self {
            Color::Blue => "blue",
            Color::Red => "red",
        });
    }
}

struct ShippingBox {
    dimensions: Dimensions,
    weight: i32,
    color: Color,
}

impl ShippingBox {
    fn create_new(width: i32, height: i32, depth: i32, weight: i32, color: Color) -> Self {
        let dimensions = Dimensions { width, height, depth };
        Self { dimensions, weight, color }
    }

    fn print(&self) {
        self.dimensions.print();
        println!("weight: {}", self.weight);
        self.color.print();
    }
}
