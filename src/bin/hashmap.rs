use std::collections::HashMap;

fn main() {
    let mut ages = HashMap::new();

    ages.insert("Asuna", 18);
    ages.insert("Mizuhara", 21);
    ages.insert("Shinobu", 571);

    let _contacts = HashMap::from([("Asuna", "98680712"), ("Mizuhara", "45783324")]);

    ages.entry("Asuna").or_insert(19);

    if let Some(age) = ages.get("Asuna") {
        println!("Asuna is {} years old!", age);
    } else {
        println!("Asuna doesn't exist");
    }

    match ages.get("Asuna") {
        Some(age) => println!("Asuna's age: {}", age),
        None => println!("Ofc, Asuna doesn't exist"),
    }

    if let Some(age) = ages.insert("Asuna", 20) {
        println!("Asuna's age was {}", age);
    }

    let age = ages.entry("Asuna").or_insert(0);
    *age += 1;
    println!("Asuna's age is {}", ages.get("Asuna").unwrap());

    if let Some(age) = ages.remove("Mizuhara") {
        println!("Mizuhara was {} years old", age);
    }

    match ages.remove("Mizuhara") {
        Some(age) => println!("Mizuhara is {}", age),
        None => println!("Mizuhara doesn't exist"),
    }

    match ages.contains_key("Asuna") {
        true => println!("Asuna is at {}", ages.get("Asuna").unwrap()),
        false => println!("Asuna doesn't exist too!"),
    }

    for (name, age) in ages.iter() {
        println!("{}: {:?}", name, age);
    }

    for (_, age) in ages.iter_mut() {
        *age += 1;
    }

    for name in ages.keys() {
        println!("{}", name);
    }

    for age in ages.values() {
        println!("{}", age);
    }

    println!("The number of elements in the hashmap: {}", ages.len());
    println!("Is empty?: {}", ages.is_empty());

    ages.clear();
}
