use std::io;

fn main() {
    // Start up messages
    println!("Guess the number!");
    println!("Please input your guess.");
    
    // Declare a mutable variable
    let mut guess = String::new();
    
    // Grab the input from standard in (user input). Expect a failure if something went wrong
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the incomming line.");

    println!("You guessed: {guess}");
}
