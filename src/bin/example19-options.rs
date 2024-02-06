struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let students = vec![
        Student{name: "Jack".to_string(), locker: None},
        Student{name: "Eva".to_string(), locker: Some(124)},
    ];

    for student in students {
        print!("Name: {}. ", student.name);

        match student.locker {
            Some(locker) => println!("Locker: {}", locker),
            None => println!("No locker"),
        }
    }
}