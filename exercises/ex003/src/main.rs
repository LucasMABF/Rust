use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number Game!!");

    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    println!("I  generated a random number from 1 to 100 and you will try to guess it.");

    loop {
        println!("What is your guess?");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("You win!! the number was {secret_number}!!");
                break;
            }
        }
    }
}
