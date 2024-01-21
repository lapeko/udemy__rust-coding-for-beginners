fn main() {
    let drink1 = Drink {
        flavor: Flavor::Cinnamon,
        ounce: 4.0,
    };

    let drink2 = Drink {
        flavor: Flavor::Mint,
        ounce: 6.0,
    };

    print_drink(drink1);
    print_drink(drink2);
}

struct Drink {
    flavor: Flavor,
    ounce: f64,
}

enum Flavor {
    Mint,
    Cinnamon,
    Vanilla,
}

fn print_drink(drink: Drink) {
    let mut flavor: &str;

    match drink.flavor {
        Flavor::Cinnamon => flavor = "cinnamon",
        Flavor::Mint => flavor = "mint",
        Flavor::Vanilla => flavor = "vanilla",
    }

    println!("Flavor is {}. Ounce is {}", flavor, drink.ounce);
}