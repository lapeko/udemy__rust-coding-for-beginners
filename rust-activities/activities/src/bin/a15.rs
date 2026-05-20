// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage(f32, String),
    Vip(f32, String),
    Standard(f32),
}

impl Ticket {
    fn print(&self) {
        match self {
            Self::Backstage(price, holder) => {
                println!("Category: Backstage. Price: {}. Holder: {}", price, holder)
            }
            Self::Vip(price, holder) => {
                println!("Category: Vip. Price: {}. Holder: {}", price, holder)
            }
            Self::Standard(price) => println!("Category: Standard. Price: {}", price),
        }
    }
}

fn main() {
    let tickets = vec![
        Ticket::Backstage(200.0, "Alligator".to_owned()),
        Ticket::Vip(100.0, "Gigan Limon".to_owned()),
        Ticket::Standard(100.0),
    ];
    for t in tickets {
        t.print();
    }
}
