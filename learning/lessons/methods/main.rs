#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated Function | aka Constructor
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // Methods
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let r: Rectangle = Rectangle {
        width: 10,
        height: 10,
    };
    println!("Area = {}", r.area());

    let r2: Rectangle = Rectangle::new(5, 10);

    println!("Rectangle2 := {:?}", r2);
    println!("Area = {}", r2.area());
}
