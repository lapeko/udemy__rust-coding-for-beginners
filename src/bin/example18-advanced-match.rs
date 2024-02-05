enum Ticket {
    Standard(f64),
    Backstage(f64, String),
    Vip(f64, String),
}

fn main() {
    let tickets = [
        Ticket::Standard(34.0),
        Ticket::Backstage(23.0, "First name".to_string()),
        Ticket::Vip(50.0, "Second name".to_owned()),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Standard(price) => println!("Standard ticket. Price: {}", price),
            Ticket::Backstage(price, name) => println!("Backstage ticket. Price: {}. Name: {}", price, name),
            Ticket::Vip(price, name) => println!("Vip ticket. Price: {}. Name: {}", price, name),
        }
    }
}