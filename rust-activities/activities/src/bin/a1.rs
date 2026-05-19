// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn print_full_name(first_name: &str, last_name: &str) {
    println!("{} {}", first_name, last_name)
}

fn main() {
    print_full_name("Vasya", "Pupkin")
}
