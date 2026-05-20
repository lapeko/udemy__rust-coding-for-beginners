// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct StudentAssignment {
    name: String,
    assignment: Option<i32>,
}

impl StudentAssignment {
    fn new(name: &str, assignment: Option<i32>) -> Self {
        Self {
            name: name.to_owned(),
            assignment,
        }
    }

    fn to_string(&self) -> String {
        format!(
            "StudentAssignment: {{
    name: {},
    assignment: {},
}}",
            self.name,
            match self.assignment {
                Some(n) => n.to_string(),
                None => "no assignment".to_owned(),
            }
        )
    }
}

fn main() {
    let students = vec![
        StudentAssignment::new("Marfa", None),
        StudentAssignment::new("Vasilisa", Some(4)),
    ];
    for s in students {
        println!("{}", s.to_string());
    }
}
