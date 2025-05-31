// https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html

fn main() {
    // let x = 5;
    // x = 6; // Error
    // let mut y = 5;
    // y = 6; // Okay (mutable)

    // const x = 5; // Error: small letter constant
    // const X: i32 = 5;

    // let x = 6;
    // let x = x + 1; // 7
    // {
    //     let x = x + 2; // 9 (scope)
    //     println!("x: {x}");
    // } // x is 7 (end of scope)
    // println!("x: {x}");

    // let x: i8 = 1; let x: u8 = 1;
    // let x: i16 = 1; let x: u16 = 1;
    // let x: i32 = 1; let x: u32 = 1;
    // let x: i64 = 1; let x: u64 = 1;
    // let x: i128 = 1; let x: u128 = 1;
    // let x: isize = 1; let x: usize = 1;
    // let y: f32 = 1.0;
    // let y: f64 = 1.0;
    // let z: bool = true;
    // let w: char = 'a';
    // let v: (char, bool, f32) = ('b', false, 2.0);
    // let v_c: char = v.0;
    // let v_b: bool = v.1;
    // let v_f: f32 = v.2;
    // let xx: [i32; 5] = [1, 2, 3, 4, 5];
    // let xx: [i32; 5] = [3; 5]; // [3, 3, 3, 3, 3]

    // fn func() {
    //     println!("another hello world...!");
    // }
    // println!("hello world");
    // func();

    // fn func(x: i32) {
    //     println!("The value of x: {x}");
    // }
    // func(5);

    // fn func(x: i32, y: char) {
    //     println!("The value of x and y: {x} {y}")
    // }
    // func(5, 'h');

    // let y = 6;
    // let x = (let y = 6);
    // let x = let y = 6;

    // let y = {
    //     let x = 3;
    //     x + 1
    // };
    // println!("The value of y: {y}");

    // fn func() -> i32 {
    //     5
    // }
    // let x = func();
    // println!("The value of x: {x}");

    // fn func(x: i32) -> i32 {
    //     x + 1
    // }
    // let x = func(5);
    // println!("The value of x: {x}");

    // let x = 3;
    // if x < 5 {
    //     println!("cond true");
    // } else {
    //     println!("cond false");
    // }
    // let y = 3;
    // if y != 0 {
    //     println!("y isn't zero");
    // }
    
    // let cond = true;
    // let x = if cond { 5 } else { 6 };
    // println!("The value of x: {x}");
    // let y = if cond {5} else { '5' }; // Error
    // println!("The value of y: {y}");

    // loop {
    //     println!("again!");
    // }

    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("The result is {result}");

    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count {count}");
    //     let mut remaining = 10;
    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }
    //     count += 1;
    // }
    // println!("End count = {count}");

    // let mut number = 3;
    // while number != 0 {
    //     println!("{number}");
    //     number -= 1;
    // }
    // println!("LIFEOFF!!!");

    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;
    // while index < 5 {
    //     println!("The value is {}", a[index]);
    //     index += 1;
    // }

    // let a = [10, 20, 30, 40, 50];
    // for el in a {
    //     println!("The value is: {el}");
    // }

    for v in (1..4).rev() {
        println!("{v}");
    }
    println!("LIFEOFF!!!");
}
