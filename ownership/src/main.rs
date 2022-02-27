fn main() {
    let s = String::from("hello💀");
    println!("{}",s.capacity());

    // Cloning
    let mut x = s.clone();
    println!("x is {}", x);

    x.push_str("New x");

    println!("X is {} s is {}", x, s);

    // References
    let capacity_minus_len = calculate_len_capacity_delta(&x);
    println!("capacity - len of X is {} - {} = {}", x.capacity(), x.len(), capacity_minus_len);

    let a = [1, 2, 3, 4];
 
    let slice = &a[0..1];

    println!("The slice is {:?}", slice);

}

fn calculate_len_capacity_delta(s:&String)-> usize{
    s.capacity() - s.len()
}