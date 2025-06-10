fn main() {
    let value = 324;
    match value {
        1 => println!("one"),
        2 | 3 => println!("two or three"),
        2..=4 => println!("2 through 4"),
        x if x>5 => println!("greater than five"),
        x @ 12..=20 => println!("twelve through twenty: {}", x),
        _ => println!("something else")
    }

}
