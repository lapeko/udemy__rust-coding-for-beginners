// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn print_bool(b: bool) {
    match b {
        true => println!("it's true"),
        _ => println!("it's false"),
    }
}

fn main() {
    print_bool(true);
    print_bool(false);
}
