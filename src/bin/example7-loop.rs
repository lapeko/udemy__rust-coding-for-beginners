fn main() {
    let mut counter = 1;

    loop {
        println!("{}", counter);

        if counter >= 4 {
            break;
        }

        counter += 1;
    }
}