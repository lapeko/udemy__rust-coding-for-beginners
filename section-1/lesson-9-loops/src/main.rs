fn main() {
    let mut i = 0;
    loop {
        if i >= 5 {
            break;
        }
        i = i + 1;
    }
    println!("{:?}", i);

    i = 0;
    while i < 5 {
        i = i + 1;
    }

    println!("{:?}", i);
}
