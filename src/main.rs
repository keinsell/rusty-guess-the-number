use std::io;
use rand::prelude::*;

fn main() {
    // Define a variable to store the user input
    let mut user_input: i16;
    // const DEFAULT_RANGE: (i16, i16) = (1, 100);

    // Prompt the user to enter the value
    println!("Enter a number: ");

    // Read the input from the user and store it in the variable
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input into an i16 value
    user_input = match input.trim().parse::<i16>() {
        Ok(value) => value,
        Err(_) => {
            panic!("Invalid input!");
        }
    };

    // Generate random number
    let mut rng = rand::thread_rng();
    let random_number: f64 = rng.gen();
    let random_number_int = (random_number * 100.0) as i16;

    // Print the random number
    println!("Random number: {}", random_number_int);

    if user_input == random_number_int {
        println!("You guessed it!");
    } else {
        println!("You didn't guess it!");
    }

}
