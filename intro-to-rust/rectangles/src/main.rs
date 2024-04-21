// version 1
// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!("Area = {} square pixels", area(width1, height1));
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// version 2
// fn main() {
//     let rect1 = (30, 50);
//     println!("Area = {} square pixels", area(rect1));
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// version 3
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     println!("Area = {} square pixels", area(&rect1));

//     println!("rect1 is {:?}", rect1);
//     println!("rect1 is {:#?}", rect1);
//     dbg!(&rect1);
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// version 4
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // usually just &self
    fn area(self: &Self) -> u32 {
        self.width * self.height
    }

    // getter
    // if field is private
    fn width(&self) -> u32 {
        self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }

    // constructor way
    // fn new(params) -> Self {Self{};}

    // specialized constructor
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    // allowed but not really needed
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 100,
    };

    println!("Area = {} square pixels", rect1.area());

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
    dbg!(&rect1);

    println!("{}", rect1.width());

    println!("can r1 hold r2? {}", rect1.can_hold(&rect2));
    println!("can r1 hold r3? {}", rect1.can_hold(&rect3));
}
