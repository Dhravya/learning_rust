use std::io;

fn main() {
    println!("Please input the number to be squared");

    let mut number_input = String::new();

    io::stdin()
        .read_line(&mut number_input)
        .expect("Invalid input");

    let number_input :i32 = number_input.trim().parse().expect("Invalid input");

    let sqr = square(number_input);
    print!("{}", sqr)
}

fn square(x: i32) -> i32 {
    x*x
}