fn main() {
    let tup: (i32, char, u8) = (23, '\0', 23);
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);

    let waha = tup.0;
    let haha = tup.1;
    let hehe = tup.2;
    println!("waha: {}, haha: {}, hehe: {}", waha, haha, hehe);
}
