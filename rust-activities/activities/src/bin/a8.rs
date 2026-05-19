// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavour {
    Sweet,
    Salty,
    Sour,
    Bitter,
    Umami,
}

fn stringify_flavour(f: Flavour) -> &'static str {
    match f {
        Flavour::Sweet => "Sweet",
        Flavour::Salty => "Salty",
        Flavour::Sour => "Sour",
        Flavour::Bitter => "Bitter",
        Flavour::Umami => "Umami",
    }
}

struct Drink {
    flavor: Flavour,
    ounce: i32,
}

fn print_drink(d: Drink) {
    println!(
        "flavor {:?}, ounce: {:?}",
        stringify_flavour(d.flavor),
        d.ounce
    );
}

fn main() {
    print_drink(Drink {
        flavor: Flavour::Sweet,
        ounce: 14,
    });
}
