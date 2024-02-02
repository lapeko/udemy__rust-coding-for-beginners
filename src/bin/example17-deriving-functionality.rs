#[derive(Debug, Copy, Clone)]
enum Color {
    Red,
    Blue,
}

#[derive(Debug, Copy, Clone)]
struct Person {
    age: i32,
    favorite_color: Color,
}

fn main() {
    let person = Person {age: 35, favorite_color: Color::Red};
    print_person(person);
    print_person(person);
}

fn print_person(person: Person) {
    println!("{:?}", person);
}