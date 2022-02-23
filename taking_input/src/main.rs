use std::io;

fn main() {
    println!("Please input your guess.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("You guessed: {}", input);
}