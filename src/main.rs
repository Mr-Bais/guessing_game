use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
    let mut rng = rand::thread_rng();
    let secret_number: u32 = rng.gen_range(1..=100);

    loop {
        // The game will prompt the user to guess a number between 1 and 100.
        println!("Guess a number between 1 ad 100");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        println!("guess is {} {}", guess, secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("big"),
            Ordering::Equal => {
                println!("win");
                break;
            }
        }
    }
}
