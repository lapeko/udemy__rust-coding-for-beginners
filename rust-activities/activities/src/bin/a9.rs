// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn print_y(coords: (i32, i32)) {
    let (_, y) = coords;
    println!(
        "{:?}",
        match y {
            n if n > 5 => "greater than 5",
            n if n < 5 => "less than 5",
            _ => "equal to 5",
        }
    )
}

fn main() {
    print_y((1, 2));
    print_y((1, 6));
    print_y((1, 5));
}
