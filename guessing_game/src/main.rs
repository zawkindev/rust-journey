use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {

    println!("\n####          Number Guessing Game          ####\n");

    let secret_number = thread_rng().gen_range(1..=3);

    loop {
        println!("Guess the secret number (1,2,3): ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("failed");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("should be greater\n"),
            Ordering::Greater => println!("should be less\n"),
            Ordering::Equal => {
                println!("you win!\n");
                break;
            }
        };
    }
}
