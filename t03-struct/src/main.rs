// https://doc.rust-lang.org/book/ch05-00-structs.html

fn main() {
    // struct User {
    //     active: bool,
    //     username: String,
    //     email: String,
    //     sign_in_count: u64,
    // }
    // fn build_user(email: String, username: String) -> User {
    //     User {
    //         active: true,
    //         username,
    //         email,
    //         sign_in_count: 1
    //     }
    // }
    // let mut user1 = User {
    //     active: true,
    //     username: String::from("someusername123"),
    //     email: String::from("someone@example.com"),
    //     sign_in_count: 1
    // };
    // // user1.email = String::from("anotheremail@example.com");
    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     ..user1
    // };

    // struct Color(i32, i32, i32);
    // struct Point(i32, i32, i32);
    // let black = Color(0, 0, 0);
    // let point = Point(0, 0, 0);

    // struct AlwaysEqual;
    // let subject = AlwaysEqual;

    // struct User {
    //     active: bool,
    //     username: &str, // Error
    //     email: &str, // Error
    //     sign_in_count: u64,
    // }

    // fn area(width: u32, height: u32) -> u32 {
    //     width * height
    // }
    // let width1 = 30;
    // let height1 = 50;
    // println!("The area of the rectangle is {} square pixels.", area(width1, height1));

    // fn area(dimensions: (u32, u32)) -> u32 {
    //     dimensions.0 * dimensions.1
    // }
    // let rect1 = (30, 50);
    // println!("The area of the rectangle is {} square pixels.", area(rect1));

    // #[derive(Debug)]
    // struct Rectangle {
    //     width: u32,
    //     height: u32
    // }
    // fn area(rectangle: &Rectangle) -> u32 {
    //     rectangle.width * rectangle.height
    // }
    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50
    // };
    // println!("The area of the rectangle is {} square pixels.", area(&rect1));
    // println!("rect1 is {rect1:?}");
    
    // #[derive(Debug)]
    // struct Rectangle {
    //     width: u32,
    //     height: u32
    // }
    // let scale = 2;
    // let rect1 = Rectangle { width: dbg!(30 * scale), height: 50 };
    // dbg!(&rect1);

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32
    }
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn width(&self) -> bool {
            self.width > 0
        }
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size
            }
        }
    }
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
    let rect2 = Rectangle {
        width: 10,
        height: 40
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45
    };
    println!("\nCan rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    let rect4 = Rectangle::square(15);
    println!("\nHere is a square rect4 (width: {}, heigth: {})", rect4.width, rect4.height);
    println!("Can rect1 hold rect4? {}", rect1.can_hold(&rect4));
}
