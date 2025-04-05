fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        // area(&rect)
        rect.area()
    );
    println!("rect is {:#?}", rect);
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));
    let sq = Rectangle::square(3);
    println!("sq is {:#?}", sq);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
