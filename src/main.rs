use rand::Rng;
use std::{cmp::Ordering, io::{self, Write}};

fn main() {
    // Declare a mutable variable
    // Generate a random number
    let random_number = rand::thread_rng().gen_range(1..=100);

    // Start up messages
    println!("Guess the number!");
    println!("The Generated Number Was: {random_number}");

    loop {
        let mut guess = String::new();
        print!("Please input your guess: ");
        io::stdout().flush().expect("Something went wrong flushing the stdout"); // Helps collect
                                                                                 // data with
                                                                                 // prompt inline
        // Grab the input from standard in (user input). Expect a failure if something went wrong
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the incomming line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please pick a number!");
                continue;
            }
        };

        match guess.cmp(&random_number) {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("To big!"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}
