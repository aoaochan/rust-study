// https://doc.rust-lang.org/book/ch10-00-generics.html

// use std::fmt::Display;

fn main() {
    // fn longest_with_an_announcement<'a, T>(
    //     x: &'a str,
    //     y: &'a str,
    //     ann: T
    // ) -> &'a str where T: Display {
    //     println!("Annnouncement! {ann}");
    //     if x.len() > y.len() { x } else { y }
    // }
}

// fn main() {
//     // fn largest(list: &[i32]) -> &i32 {
//     //     let mut largest = &list[0];
//     //     for v in list {
//     //         if v > largest {
//     //             largest = v;
//     //         }
//     //     }
//     //     largest
//     // }
//     // let number_list = vec![34, 50, 25, 100, 65];
//     // println!("The largest number is {}", largest(&number_list));
//     // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
//     // println!("The largest number is {}", largest(&number_list));

//     // fn largest<T>(list: &[T]) -> &T {
//     //     let mut largest = &list[0];
//     //     for v in list {
//     //         if v > largest { // Error
//     //             largest = v;
//     //         }
//     //     }
//     //     largest
//     // }
//     // let number_list = vec![34, 50, 25, 100, 65];
//     // println!("The largest number is {}", largest(&number_list));
//     // let char_list = vec!['y', 'm', 'a', 'q'];
//     // println!("The largest char is {}", largest(&char_list));

//     // struct Point<T> { x: T, y: T }
//     // let integer = Point { x: 5, y: 10 };
//     // let float = Point { x: 1.0, y: 4.0 }; 

//     // struct Point<T, U> { x: T, y: U }
//     // let both_integer = Point { x: 5, y: 10 };
//     // let both_float = Point { x: 1.0, y: 4.0 };
//     // let integer_and_float = Point { x: 5, y: 4.0 };

//     // struct Point<T> { x: T, y: T }
//     // impl<T> Point<T> {
//     //     fn x(&self) -> &T {
//     //         &self.x
//     //     }
//     // }
//     // let p = Point { x: 5, y: 10 };
//     // println!("p.x = {}", p.x());

//     // struct Point<T, U> { x: T, y: U }
//     // impl<T, U> Point<T, U> {
//     //     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//     //         Point {
//     //             x: self.x,
//     //             y: other.y
//     //         }
//     //     }
//     // }
//     // let p1 = Point { x: 5, y: 10.4 };
//     // let p2 = Point { x: "Hello", y: 'c' };
//     // let p3 = p1.mixup(p2);
//     // println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

//     // trait Summary {
//     //     fn summarize_author(&self) -> String;
//     //     fn summarize(&self) -> String {
//     //         format!("(Read more from {}...)", self.summarize_author())
//     //     }
//     // }
//     // // struct NewsArticle {
//     // //     headline: String,
//     // //     location: String,
//     // //     author: String,
//     // //     content: String
//     // // }
//     // // impl Summary for NewsArticle {
//     // //     fn summarize(&self) -> String {
//     // //         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     // //     }
//     // // }
//     // struct SocialPost {
//     //     username: String,
//     //     content: String,
//     //     reply: bool,
//     //     repost: bool
//     // }
//     // impl Summary for SocialPost {
//     //     fn summarize_author(&self) -> String {
//     //         format!("@{}", self.username)
//     //     }
//     //     // fn summarize(&self) -> String {
//     //     //     format!("{}: {}", self.username, self.content)
//     //     // }
//     // }
//     // pub fn notify<T: Summary>(item: &T) {
//     //     println!("Breaking news! {}", item.summarize());
//     // }
//     // // let post = SocalPost {
//     // //     username: String::from("horse_ebooks"),
//     // //     content: String::from("of course, as you probably already know, people"),
//     // //     reply: false,
//     // //     repost: false
//     // // };
//     // // println!("1 new socal post: {}", post.summarize());
//     // // let article = NewsArticle {
//     // //     headline: String::from("Penguins win the Stanley Cup Championship!"),
//     // //     location: String::from("Pittsburgh, PA, USA"),
//     // //     author: String::from("Iceburgh"),
//     // //     content: String::from(
//     // //         "The Pittsburgh Penguins once again are the best \
//     // //          hockey team in the NHL.",
//     // //     ),
//     // // };
//     // // println!("New article available! {}", article.summarize());
//     // let post = SocialPost {
//     //     username: String::from("horse_ebooks"),
//     //     content: String::from(
//     //         "of course, as you probably already know, people",
//     //     ),
//     //     reply: false,
//     //     repost: false,
//     // };
//     // println!("1 new social post: {}", post.summarize());

//     // fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     //     if x.len() > y.len() { x } else { y }
//     // }
//     // // let s1 = String::from("abcd");
//     // // let s2 = "xyz";
//     // // let result = longest(s1.as_str(), s2);
//     // // println!("The longest string is {result}");
//     // let string1 = String::from("long string is long");
//     // {
//     //     let string2 = String::from("xyz");
//     //     let result = longest(string1.as_str(), string2.as_str());
//     //     println!("The longest string is {result}");
//     // }

//     // struct ImportantExcerpt<'a> {
//     //     part: &'a str
//     // }
//     // let novel = String::from("Call me Ishmael. Some years ago...");
//     // let first_sentence = novel.split('.').next().unwrap();
//     // let i = ImportantExcerpt {
//     //     part: first_sentence
//     // };
   
// }