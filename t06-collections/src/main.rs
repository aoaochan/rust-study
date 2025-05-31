// https://doc.rust-lang.org/book/ch08-00-common-collections.html

use std::collections::HashMap;

fn main() {
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];
    
    // let mut v = Vec::new();
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);

    // let v = vec![1, 2, 3, 4, 5];
    // let third: &i32 = &v[2];
    // println!("The third el is {third}.");
    // let third: Option<&i32> = v.get(2);
    // match third {
    //     Some(third) => println!("The third el is {third}."),
    //     None => println!("There is no third el.")
    // }

    // let v = vec![1, 2, 3, 4, 5];
    // let a = &v[100];
    // let a = v.get(100);

    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6); // Error
    // println!("The first El is: {first}");

    // let mut v = vec![100, 32, 57];
    // for i in &mut v {
    //     *i += 50;
    //     println!("{i}");
    // }

    // let data = "hello world";
    // let s1 = data.to_string();
    // let s2 = "hello world".to_string();
    // let s3 = String::from("hello world");

    // let mut s = String::from("foo");
    // s.push_str("bar");

    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("s2 is {s2}");

    // let mut s = String::from("lo");
    // s.push('l');

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1.clone() + &s2;
    // println!("{s1} {s2} | {s3}");

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    // // let s = s1 + "-" + &s2 + "-" + &s3;
    // let s = format!("{}-{}-{}", s1, s2, s3);

    // let s1 = "hello";
    // for c in s1.chars() {
    //     println!("{c}");
    // }

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // // scores.insert(String::from("Yellow"), 50);
    // // let team_name = String::from("Blue");
    // // let score = scores.get(&team_name).copied().unwrap_or(0);
    // // for (k, v) in &scores {
    // //     println!("{k} {v}");
    // // }
    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);
    // println!("{scores:?}");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");
}
