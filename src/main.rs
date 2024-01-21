fn main() {
    let mut counter = 0;
    loop {
        if counter >= 5 {
            break;
        }
        println!("Hello, world!");
        counter += 1;
    }
}
