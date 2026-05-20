// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Grocery {
    id: i32,
    quantity: i32,
}

fn display_grocery_id(g: &Grocery) {
    println!("ID: {:?}", g.id)
}

fn display_grocery_quantity(g: &Grocery) {
    println!("Quantity: {:?}", g.quantity)
}

fn main() {
    let g = Grocery { id: 1, quantity: 2 };
    display_grocery_id(&g);
    display_grocery_quantity(&g);
}
