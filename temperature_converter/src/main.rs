use std::io;

fn main() {
    println!("Temperature converter!");
    println!("Enter the degree in celsius: ");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Unable to take input");

    let user_input: f64 = user_input.trim().parse().expect("Invalid input");

    println!(
        "{} degree celsius = {} degree farenhiet",
        user_input,
        celsius_to_f(user_input)
    )
}

fn celsius_to_f(celsius: f64) -> f64 {
    (celsius * 1.8) + 32.00
}
