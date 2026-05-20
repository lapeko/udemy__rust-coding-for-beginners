// Project 1: Interactive bill manager
//
// User stories:
// * L1: I want to add bills, including the name and amount owed.
// * L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at level 1, but a
//   hashmap will be easier to work with at levels 2 and 3.
// * Create a function just for retrieving user input, and reuse it
//   throughout your program.
// * Create your program starting at level 1. Once finished, advance to the
//   next level.

use std::collections::HashMap;
use std::io;

struct Bill {
    name: String,
    amount_history: Vec<i32>,
}

impl Bill {
    fn new(name: String, amount: i32) -> Self {
        Self {
            name,
            amount_history: vec![amount],
        }
    }

    fn update_umount(&mut self, amount: i32) {
        self.amount_history.push(amount)
    }

    fn get_data(&self) -> (String, i32) {
        (self.name.clone(), *self.amount_history.last().unwrap())
    }
}

struct Bills {
    data: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn add(&mut self, name: String, amount: i32) -> Result<(), String> {
        if self.data.contains_key(&name) {
            return Err(format!("Bill with name {:?} already exists", name));
        }
        self.data.insert(name.clone(), Bill::new(name, amount));
        Ok(())
    }

    fn view(&self) {
        println!("Printing your bills...");
        for bill in self.data.values() {
            let (name, amount) = bill.get_data();
            println!("Name: {:?}. Amount: {:?}", name, amount);
        }
        println!("Done");
    }

    fn remove(&mut self, name: String) -> Result<(), String> {
        if !self.data.contains_key(&name) {
            return Err(format!("Bill with name {:?} does not exist", name));
        }
        self.data.remove(&name);
        Ok(())
    }

    fn edit(&mut self, name: String, amount: i32) -> Result<(), String> {
        if let Some(bill) = self.data.get_mut(&name) {
            bill.update_umount(amount);
            Ok(())
        } else {
            Err(format!("Bill with name '{}' does not exist", name))
        }
    }

    fn go_back(&mut self, name: String) -> Result<(), String> {
        if let Some(bill) = self.data.get_mut(&name) {
            if bill.amount_history.len() > 1 {
                bill.amount_history.pop();
                Ok(())
            } else {
                Err(format!(
                    "Cannot go back: '{}' has no previous history",
                    name
                ))
            }
        } else {
            Err(format!("Bill with name '{}' does not exist", name))
        }
    }
}

struct EngineCLI {}

impl EngineCLI {
    fn new() -> Self {
        EngineCLI {}
    }

    fn print_menu(&self) {
        println!(
            "1. add
2. view
3. remove
4. edit
5. go_back
0. exit"
        )
    }

    fn read(&self) -> io::Result<String> {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        Ok(buffer.trim().to_owned())
    }

    fn read_command(&self) -> CLICommand {
        println!("Enter a number of the operation:");

        loop {
            let s = match self.read() {
                Ok(s) => s,
                Err(e) => {
                    println!("Failed to read input. Error: {:?}", e);
                    continue;
                }
            };

            match s.parse::<i32>() {
                Ok(n) => {
                    use CLICommand::*;
                    return match n {
                        1 => Add,
                        2 => View,
                        3 => Remove,
                        4 => Edit,
                        5 => GoBack,
                        0 => Exit,
                        _ => {
                            println!("You provided not correct number: {:?}", s);
                            continue;
                        }
                    };
                }
                Err(_) => {
                    println!("You provided not a number: {:?}", s);
                }
            }
        }
    }

    fn read_name(&self) -> String {
        loop {
            println!("Enter bill name");
            match self.read() {
                Ok(s) => return s,
                Err(e) => println!("Failed to read input. Error: {:?}", e),
            }
        }
    }

    fn read_amount(&self) -> i32 {
        loop {
            println!("Enter bill amount");

            match self.read() {
                Ok(s) => match s.parse::<i32>() {
                    Ok(n) => return n,
                    Err(_) => println!("You provided not a number: {:?}", s),
                },
                Err(e) => println!("Failed to read input. Error: {:?}", e),
            }
        }
    }
}

#[derive(Debug)]
enum CLICommand {
    Add,
    View,
    Remove,
    Edit,
    GoBack,
    Exit,
}

struct Engine {
    data: Bills,
    cli: EngineCLI,
}

impl Engine {
    fn new() -> Self {
        Self {
            data: Bills::new(),
            cli: EngineCLI::new(),
        }
    }

    fn run(&mut self) {
        loop {
            use CLICommand::*;

            self.cli.print_menu();
            match self.cli.read_command() {
                Add => self.add(),
                View => self.data.view(),
                Remove => self.remove(),
                Edit => self.edit(),
                GoBack => self.go_back(),
                Exit => {
                    println!("Exit program...");
                    break;
                }
            };
        }
    }

    fn add(&mut self) {
        let name = self.cli.read_name();
        let amount = self.cli.read_amount();
        match self.data.add(name, amount) {
            Ok(()) => println!("Success"),
            Err(e) => println!("Error: {:?}", e),
        };
    }

    fn remove(&mut self) {
        let name = self.cli.read_name();
        match self.data.remove(name) {
            Ok(()) => println!("Success"),
            Err(e) => println!("Error: {:?}", e),
        };
    }

    fn edit(&mut self) {
        let name = self.cli.read_name();
        let amount = self.cli.read_amount();
        match self.data.edit(name, amount) {
            Ok(()) => println!("Success"),
            Err(e) => println!("Error: {:?}", e),
        };
    }

    fn go_back(&mut self) {
        let name = self.cli.read_name();
        match self.data.go_back(name) {
            Ok(()) => println!("Success"),
            Err(e) => println!("Error: {:?}", e),
        };
    }
}

fn main() {
    Engine::new().run();
}
