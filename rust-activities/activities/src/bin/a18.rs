// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    age: i32,
}

const LEGAL_AGE: i32 = 21;
impl Customer {
    fn new(age: i32) -> Self {
        Self { age }
    }

    fn purchase(&self) -> Result<(), String> {
        if self.age >= LEGAL_AGE {
            return Ok(());
        }
        Err(format!("Purchase not allowed for age under {}", LEGAL_AGE))
    }
}

fn main() {
    let customers = vec![Customer::new(18), Customer::new(21)];
    for c in customers {
        match c.purchase() {
            Err(reason) => println!("{}", reason),
            Ok(()) => println!("Ok"),
        };
    }
}
