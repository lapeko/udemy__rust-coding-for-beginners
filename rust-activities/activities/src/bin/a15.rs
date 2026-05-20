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

enum TicketCategory {
    Backstage(String),
    Vip(String),
    Standard,
}

impl TicketCategory {
    fn to_string(&self) -> String {
        match self {
            Self::Backstage(name) => format!("Backstage, Holder: {}", name),
            Self::Vip(name) => format!("Vip, Holder: {}", name),
            Self::Standard => "Standard".to_owned(),
        }
    }
}

struct Ticket {
    category: TicketCategory,
    price: f32,
}

impl Ticket {
    fn new(category: TicketCategory, price: f32) -> Self {
        Self { category, price }
    }

    fn to_string(&self) -> String {
        format!(
            "Ticket {{
    category: {},
    price: {},
}}",
            self.category.to_string(),
            self.price
        )
    }
}

fn main() {
    let tickets = vec![
        Ticket::new(TicketCategory::Backstage("Alligator".to_owned()), 200.0),
        Ticket::new(TicketCategory::Vip("Gigan Limon".to_owned()), 100.0),
        Ticket::new(TicketCategory::Standard, 100.0),
    ];
    for t in tickets {
        println!("{}", t.to_string())
    }
}
