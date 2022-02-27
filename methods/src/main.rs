#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn diagonal(&self) -> f32 {
        (self.width as f32).hypot(self.height as f32)
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20, 
        height: 40
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "The diagonal of rectangle is {} pixels",
        rect1.diagonal()
    );

    println!(
        "Can rect1 hold rect2? : {}",
        rect1.can_hold(&rect2)
    );

    let sq = Rectangle::square(20);
    println!("{:?}", sq)
}