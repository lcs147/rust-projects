use std::io::{self, Write};
use rand::Rng;

fn main() {
    let secret = rand::rng().random_range(1..=100);

    loop {
        print!("Guess a number: ");
        io::stdout().flush().expect("Failed to flush!");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read");
        let guess: i32 = match guess.trim().parse() {
            Ok(guess) => guess,
            Err(_) => continue,
        };

        if guess < secret {
            println!("Too small!");
        } else if guess > secret {
            println!("Too big!");
        } else {
            println!("Found it!");
            break;
        }
    }
    
}
