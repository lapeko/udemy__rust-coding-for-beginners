struct User {
    name: String,
    age: i32,
}

fn can_make_purchase(user: &User) -> Result<(), String> {
    match user.age >= 21 {
        true => Ok(()),
        _ => Err("Restricted by age".to_owned()),
    }
}

fn purchase(user: &User) -> Result<(), String> {
    can_make_purchase(user)?;
    println!("Purchasing... User: {}", user.name);
    Ok(())
}

fn main() {
    let user = User {name: "Alesia".to_owned(), age: 20};
    match purchase(&user) {
        Err(error) => println!("Error: {}", error),
        _ => (),
    }
}