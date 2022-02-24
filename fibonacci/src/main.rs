fn main() {
   for elem in 1..10 {
       println!("{}", fibonacci(elem))
   }
}

fn fibonacci(n:u32) -> u32 {
    if n <= 0 {
        println!("Invalid input")
    }
    else if n == 0 {
        return 0
    }
    else if n == 1 || n == 2 {
        return 1
    }
    else{
        return fibonacci(n-1) + fibonacci(n-2)
    }
    n
}