// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let furniture_store =
        HashMap::from([("Chairs", 5), ("Beds", 3), ("Tables", 2), ("Couches", 0)]);

    let mut total_stock = 0;
    for (name, count) in furniture_store {
        total_stock = total_stock + count;
        let str_count = match count {
            0 => "out of stock".to_owned(),
            n => format!("{}", n),
        };
        println!("name: {:?}. count: {:?}", name, str_count);
        println!("Total stock: {:?}", total_stock);
    }
}
