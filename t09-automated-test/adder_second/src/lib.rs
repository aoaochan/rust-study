#[cfg(test)]
mod test {
    fn greeting(name: &str) -> String {
        format!("Hello {name}!") // ok
        // String::from("Hello!") // FAILED
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("aoaochan");
        assert!(
            result.contains("aoaochan"),
            "Greeting did not contain name, value was '{result}'"
        );
    }
}

// #[cfg(test)]
// mod test {
//     fn add_two(a: usize) -> usize {
//         a + 2 // ok
//         // a + 3 // FAILED
//     }

//     #[test]
//     fn it_adds_two() {
//         let result = add_two(2);
//         assert_eq!(result, 4);
//     }
// }