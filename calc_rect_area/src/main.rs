fn main() {
    println!("Area as Standalone Vars");
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area_as_standalone_vars(width1, height1)
    );
    println!();

    println!("Area as Tuple");
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_as_tuple(rect1)
    );
    println!();

    println!("Area as Struct");
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_as_struct(&rect2)
    );
    println!("rect2 is {:#?}", rect2);
    println!();

    println!("Area as Struct Method");
    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect3.area()
    );
    println!();

    println!("Can Hold");
    let rect4 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect5 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect3 hold rect4? {}", rect3.can_hold(&rect4));
    println!("Can rect3 hold rect5? {}", rect3.can_hold(&rect5));
    println!();

    println!("Square");
    let square = Rectangle::square(3);
    println!("square: {:#?}", square);
    println!("The area of the square is {} square pixels.", square.area());
}

fn area_as_standalone_vars(width: u32, height: u32) -> u32 {
    width * height
}

fn area_as_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_as_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

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
