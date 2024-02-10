use std::collections::HashMap;

fn main() {
    let furniture_map = HashMap::from([
        ("Chair", 0),
        ("Bed", 2),
        ("Table", 100),
    ]);

    let mut total_quantity = 0;

    for (key, count) in &furniture_map {
        match count {
            0 => println!("Key: \"{}\". Out of stock", key),
            _count => println!("Key: \"{}\". Count: {}", key, _count),
        }
        total_quantity += count;
    }

    // for (key, quantity) in &furniture_map {
    //     let line = if quantity == &0 {
    //         "Out of stock".to_owned()
    //     } else {
    //         format!("Quantity: {}", quantity)
    //     };
    //     println!("Name: {}. {}", key, line);
    //     total_quantity += quantity;
    // }

    println!("Total count: {}", total_quantity);
}