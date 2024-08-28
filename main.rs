use rand::Rng;
use std::io;

fn main() {
    let correct = rand::thread_rng().gen_range(1..=10);
    println!("correct: {corrext}"); // corrected string interpolation
    println!("Hey, guess a number 1-10:");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading input");

    // Proper error handling for parsing
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    if correct == guess {
        println!("You guessed the correct number!");
    } else {
        println!("You guessed the wrong number. Try again!");
    }
}
