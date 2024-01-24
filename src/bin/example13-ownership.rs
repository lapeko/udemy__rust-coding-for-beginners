fn main() {
    let item = GroceryItem { id: 1, quantity: 4 };
    print_grocery_id(&item);
    print_grocery_quantity(&item);
}

struct GroceryItem {
    id: i32,
    quantity: i32,
}

fn print_grocery_id(grocery: &GroceryItem) {
    println!("id: {}", grocery.id);
}

fn print_grocery_quantity(grocery: &GroceryItem) {
    println!("quantity: {}", grocery.quantity);
}