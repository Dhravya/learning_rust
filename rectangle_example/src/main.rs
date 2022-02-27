#[derive(Debug)]
struct Rectangle {
    width : u32,
    height: u32
}

fn main() {
    let rect1 = Rectangle{
        width: 30, 
        height: 40
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    dbg!(&rect1);
}

fn area(rect :&Rectangle) -> u32 {
    rect.width * rect.height
}