use std::io;

fn main() {
    let mut input: String = String::new();

    println!("Enter a string: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("You have entered: {}", input);
}
