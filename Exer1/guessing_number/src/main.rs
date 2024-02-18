use std::io::{self, Write};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();

    loop {
        print!("Please input your guess: ");
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        if let Ok(guess_number) = guess.trim().parse::<i32>() {
            if guess_number > secret_number {
                println!("Your guess {} is too high!", guess_number);
            }
            else if guess_number < secret_number {
                println!("Your guess {} is too low!", guess_number);
            }
            else {
                println!("You guessed it!");
                break;
            }
        }
        guess.clear();
    }
}
