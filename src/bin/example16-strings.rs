enum Color {
    Red,
    Blue,
    Purple,
}

fn main() {
    let persons = vec![
        Person { name: "Vasili".to_owned(), age: 23, favorite_color: Color::Red },
        Person { name: "Maria".to_owned(), age: 4, favorite_color: Color::Blue },
        Person { name: "Igor".to_owned(), age: 12, favorite_color: Color::Red },
        Person { name: "Vitali".to_owned(), age: 6, favorite_color: Color::Purple },
        Person { name: "Anna".to_owned(), age: 34, favorite_color: Color::Purple },
        Person { name: String::from("Ewa"), age: 1, favorite_color: Color::Blue },
        Person { name: String::from("Ekaterina"), age: 56, favorite_color: Color::Red },
        Person { name: String::from("Olga"), age: 3, favorite_color: Color::Red },
        Person { name: String::from("Andrew"), age: 8, favorite_color: Color::Blue },
        Person { name: String::from("Konstantin"), age: 7, favorite_color: Color::Red },
    ];

    for person in persons {
        if person.age <= 10 {
            person.print();
        }
    }
}

struct Person {
    name: String,
    age: i32,
    favorite_color: Color,
}

impl Person {
    fn print(&self) {
        print_string("Name", &self.name);
        print_age(self.age);
        print_string("Favorite color", match self.favorite_color {
            Color::Red => "red",
            Color::Blue => "blue",
            Color::Purple => "purple",
        });
        println!();
    }
}

fn print_string(description: &str, value: &str) {
    println!("{}: {}", description, value);
}

fn print_age(age: i32) {
    println!("Age: {}", age);
}