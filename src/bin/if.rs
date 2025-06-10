fn main() {
    let result: Option<i8> = Some(32);
    if let Some(value) = result {
        print!("Result: {}", value);
    } else {
        println!("There is no value");
    }
}
