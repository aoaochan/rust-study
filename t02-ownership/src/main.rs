// https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html 

fn main() {
    // let mut s = String::from("hello");
    // s.push_str(", world!");
    // println!("{s}");

    // let x = 5;
    // let y = x;

    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{s1}"); // Error

    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("{s1} {s2}");
    // let x = 5;
    // let y = x;
    // println!("{x} {y}");

    // fn takes_ownership(s: String) {
    //     println!("{s}");
    // }
    // fn makes_copy(x: i32) {
    //     println!("{x}")
    // }
    // let s = String::from("hello");
    // takes_ownership(s);
    // println!("{s}"); // Error
    // let x = 5;
    // makes_copy(x);
    // println!("{x}"); // Okay

    // fn gives_ownership() -> String {
    //     let s = String::from("yours");
    //     s
    // }
    // fn takes_and_gives_back(s: String) -> String {
    //     s
    // }
    // let s1 = gives_ownership();
    // let s2 = String::from("hello");
    // let s3 = takes_and_gives_back(s2);

    // fn callen(s: String) -> (String, usize) {
    //     let len = s.len();
    //     (s, len)
    // }
    // let s1 = String::from("hello");
    // let (s2, len) = callen(s1);
    // println!("{s2} {len}");

    // fn callen(s: &String) -> usize {
    //     s.len()
    // }
    // let s1 = String::from("hello");
    // let len = callen(&s1);
    // println!("{s1} {len}");

    // fn change(s: &mut String) {
    //     s.push_str(", world!");
    // }
    // let mut s = String::from("hello");
    // println!("{s}");
    // change(&mut s);
    // println!("{s}");

    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s; // Error
    // println!("{r1} {r2}");

    // let mut s = String::from("hello");
    // {
    //     let r1 = &mut s;
    // }
    // let r2 = &mut s; // Okay

    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &s;
    // let r3 = &mut s; // Error
    // println!("{r1} {r2} and {r3}");

    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &s;
    // println!("{r1} {r2}");
    // let r3 = &mut s;
    // println!("{r3}");

    // fn dangle() -> &String { // Error
    //     let s = String::from("hello");
    //     &s // Error
    // }
    // fn no_dangle() -> String { // Okay
    //     let s = String::from("hello");
    //     s // Okay
    // }
    // let s1 = dangle();
    // let s2 = no_dangle();

    // fn first_word(s: &String) -> usize {
    //     // 1. String to array of bytes
    //     let bytes = s.as_bytes();

    //     // 2. create an iterator over the array of bytes
    //     for (i, &item) in bytes.iter().enumerate() {
    //         if item == b' ' {
    //             return i;
    //         }
    //     }
    //     s.len()
    // }
    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // s.clear();

    // let s = String::from("hello world");
    // let s1 = &s[..5];
    // let s2 = &s[6..s.len()];
    // let len = s.len();
    // let s3 = &s[6..len];
    // println!("{} {s2}", &s1);
    // println!("{s1} {s3}");

    // fn first_word(s: &String) -> &str {
    //     let bytes = s.as_bytes();
    //     for (i, &v) in bytes.iter().enumerate() {
    //         if v == b' ' {
    //             return &s[..i];
    //         }
    //     }
    //     &s[..]
    // }
    // let s = String::from("hello world");
    // let word = first_word(&s);
    // s.clear(); // Error
    // println!("the first word: {word}");

    // fn first_word(s: &str) -> &str {
    //     let bytes = s.as_bytes();
    //     for (i, &v) in bytes.iter().enumerate() {
    //         if v == b' ' {
    //             return &s[..i];
    //         }
    //     }
    //     &s[..]
    // }
    // let my_string = String::from("hello world");
    // // `first_word`는 `String`의 일부 또는 전체 슬라이스에 적용됩니다.
    // let word = first_word(&my_string[..6]);
    // let word = first_word(&my_string[..]);
    // // `first_word`는 `String`에 대한 참조에도 적용되며, 이는
    // // `String`의 전체 슬라이스와 동일합니다.
    // let word = first_word(&my_string);
    // let my_string_literal = "hello world";
    // // `first_word`는 문자열 리터럴의 일부 또는 전체 슬라이스에 적용됩니다.
    // let word = first_word(&my_string[..6]);
    // let word = first_word(&my_string[..]);
    // // 문자열 리터럴은 이미 문자열 슬라이스이기 때문에
    // // 슬라이스 구문 없이도 작동합니다!
    // let word = first_word(&my_string_literal);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
