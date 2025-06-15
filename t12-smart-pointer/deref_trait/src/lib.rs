#[cfg(test)]
mod test {
    use std::ops::Deref;
    struct MyBox<T>(T);
    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> { MyBox(x) }
    }
    #[test]
    fn foo() {
        fn hello(name: &str) {
            println!("Hello, {name}!");
        }
        let m = MyBox::new(String::from("Rust"));
        hello(&m);
    }
}

// #[cfg(test)]
// mod test {
//     use std::ops::Deref;
//     struct MyBox<T>(T);
//     impl<T> Deref for MyBox<T> {
//         type Target = T;
//         fn deref(&self) -> &Self::Target { &self.0 }
//         fn new(x: T) -> MyBox<T> { MyBox(x) }
//     }
//     #[test]
//     fn foo() {
//         let x = 5;
//         let y = MyBox::new(x);
//         assert_eq!(5, x);
//         assert_eq!(5, *y);
//     }
// }

// #[cfg(test)]
// mod test {
//     #[test]
//     fn foo() {
//         let x = 5;
//         let y = Box::new(x); // &x;
//         assert_eq!(5, x);
//         assert_eq!(5, *y);
//     }
// }