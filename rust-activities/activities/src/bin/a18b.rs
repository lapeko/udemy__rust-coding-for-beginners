// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum EmployeeType {
    Maintenance,
    Marketing,
    Managers,
    LineSupervisors,
    Kitchen,
    AssemblyTechnicians,
}

struct Employee {
    name: String,
    employee_type: EmployeeType,
    employed: bool,
}

impl Employee {
    fn new(name: &str, employee_type: EmployeeType, employed: bool) -> Self {
        Self {
            name: name.to_owned(),
            employee_type,
            employed,
        }
    }

    fn enter(&self) -> Result<(), String> {
        if !self.employed {
            return Err("terminated".to_owned());
        }
        match self.employee_type {
            EmployeeType::Maintenance | EmployeeType::Marketing | EmployeeType::Managers => Ok(()),
            _ => Err("invalid department".to_owned()),
        }
    }

    fn print_enter(&self) -> Result<(), String> {
        self.enter()?;
        println!("{} entered", self.name);
        Ok(())
    }
}

fn main() {
    let employees = vec![
        Employee::new("Vasyan", EmployeeType::Maintenance, false),
        Employee::new("Ex", EmployeeType::Kitchen, true),
        Employee::new("Gordon", EmployeeType::Managers, true),
    ];

    for e in employees {
        match e.print_enter() {
            Err(msg) => println!("access for {} denied. Reason: {}", e.name, msg),
            _ => (),
        };
    }
}
