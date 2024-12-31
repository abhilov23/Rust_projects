use std::io; // For input/output
use rand::Rng; // For random number generation
use std::cmp::Ordering; // For comparison
use colored::*; //for adding coloring in the output

fn main() {
    println!("Welcome to the Number Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..101); // Generate random number between 1 and 100

    loop {
        println!("Please input your guess:");

        let mut guess = String::new(); // Create a mutable, empty string

        // Reading input from the user
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // Converting input to a number, handling errors gracefully
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue; // Skip this iteration if input is invalid
            }
        };

        println!("You guessed: {}", guess);

        // Comparing the guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small!".purple()), //adding colors in the output
            Ordering::Greater => println!("{}","Too big!".purple()),
            Ordering::Equal => {
                println!("{}","You win!".green());
                break; // Exit the loop if the guess is correct
            }
        }
    }
}
