fn main() {
    let add_v1 = |a: u8, b: u8| -> u8 { a + b };
    println!("two numbers added: {}", add_v1(1, 2));

    let add_v2 = |a: i8, b: i8| a + b;
    println!("two numbers added: {}", add_v2(3, 3));

    let add_v1 = |a, b| a + b;
    println!("two numbers added: {}", add_v1(8, 3));

    let x = 1;
    let capture = |y| x + y;
    println!("capturing x: {}", capture(50));

    fn call_with_one<F>(closure: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        closure(1)
    }

    let answer = call_with_one(|x| x * x);
    println!("answer: {}", answer);
}
