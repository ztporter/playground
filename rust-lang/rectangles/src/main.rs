#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        if self.width < rect.width {
            false
        } else if self.height < rect.height {
            false
        } else {
            true
        }
    }
    fn square(length: u32) -> Rectangle {
        Rectangle { width: length, height: length }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 20, height: 40 };
    let rect3 = Rectangle { width: 30, height: 60 };
    let square = Rectangle::square(50);

    println!("rect1 is: {:?}", rect1);
    println!("rect2 is: {:?}", rect2);
    println!("rect3 is: {:?}", rect3);
    println!("square is: {:?}", square);
    println!("area is: {}", rect1.area());
    println!("area is: {}", rect2.area());
    println!("area is: {}", rect3.area());
    println!("area is: {}", square.area());
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect2 can hold rect3: {}", rect2.can_hold(&rect3));
    println!("rect3 can hold rect1: {}", rect3.can_hold(&rect1));
}

