fn main() {
    println(sum(1, 2));
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn println(num: i32) {
    println!("{:?}", num);
}
