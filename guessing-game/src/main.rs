use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("Guess the number");

    // Generate a random integer between 1 and 100
    // start..=end and is inclusive on the lower and upper bounds
    let number_to_guess = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("Please input your guess: ");
        let mut guess = String::new();

        // Flush the output so the input can be obtained from the same line as the print
        let _ = io::stdout().flush();

        // Read guess entered by user
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the user's guess into a number
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                // If the input was not a number, continue the loop to obtain the input again
                println!("Please enter a number");
                continue;
            }
        };

        // Compare the guess to the number generated
        match guess.cmp(&number_to_guess) {
            Ordering::Equal => {
                println!("You guessed the number");
                // Exit the loop to stop accepting new guesses
                break;
            }
            Ordering::Greater => println!("Your guess was too big"),
            Ordering::Less => println!("Your guess was too small"),
        }
    }
}
