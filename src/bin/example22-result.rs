enum Position {
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTechnicians,
}

struct Employee {
    name: String,
    position: Position,
    terminated: bool,
}

impl Employee {
    fn try_access(&self) -> Result<(), String> {
        if self.terminated {
            return Err("Access denied".to_owned());
        }
        match self.position {
            Position::Maintenance | Position::Marketing | Position::Manager => Ok(()),
            _ => Err("Access denied".to_owned()),
        }
    }

    fn enter(&self) -> Result<(), String> {
        self.try_access()?;
        println!("User: {}. Access allowed. Entering the building...", self.name);
        Ok(())
    }
}

fn main() {
    let eml1 = Employee {name: "Paul".to_owned(), position: Position::Maintenance, terminated: false};
    let eml2 = Employee {name: "Anna".to_owned(), position: Position::Maintenance, terminated: true};
    let eml3 = Employee {name: "George".to_owned(), position: Position::KitchenStaff, terminated: false};
    let eml4 = Employee {name: "Maria".to_owned(), position: Position::KitchenStaff, terminated: true};

    let employees = vec![
        eml1,
        eml2,
        eml3,
        eml4,
    ];

    for employee in employees {
        match employee.enter() {
            Err(_) => println!("User: {}. Access denied.", employee.name),
            _ => (),
        };
    }
}