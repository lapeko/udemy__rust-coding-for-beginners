fn main() {
    let number = 101;

    let message = match number > 100 {
        true => "its big",
        false => "its small"
    };

    println!("{}", message);
}