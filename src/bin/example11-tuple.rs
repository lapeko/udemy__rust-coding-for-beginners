fn main() {
    let coordinate = coordinate();

    print_y_coordinate(coordinate);
}

fn coordinate() -> (i32, i32) {
    (1, -3)
}

fn print_y_coordinate(coordinate: (i32, i32)) {
    let (_, y) = coordinate;

    if y > 5 {
        println!("greater than 5");
    } else if y < 5 {
        println!("less than 5");
    } else {
        println!("equal to 5");
    }
}