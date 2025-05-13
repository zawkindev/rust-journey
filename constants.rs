const GLOBAL_CONSTANT: u32 = 100_000;

fn main() {
    println!("constant: {}", GLOBAL_CONSTANT);

    const ONE:u32 = 1;
    println!("ONE: {}", ONE);

    const TUPLE: (u32, f32, bool, char) = (ONE, 3.14, true, '\0');
    println!("TUPLE: {:?}", TUPLE);

    const ARRAY: [u32; 3] = [ONE, ONE, ONE];
    println!("ARRAY: {:?}", ARRAY);

}
