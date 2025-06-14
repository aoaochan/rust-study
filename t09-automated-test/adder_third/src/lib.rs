#[cfg(test)]
mod test {
    struct Guess {
        value: i32
    }

    impl Guess {
        fn new(value: i32) -> Guess {
            // if value < 1 || value > 100 {
            //     panic!("Guess value must be between 1 and 100, got {value}.");
            // }
            if value < 1 { panic!("Guess value must be greater than or equal to 1, got {value}."); }
            else if value > 100 { panic!("Guess value must be less than or equal to 100, got {value}."); }

            Guess { value }
        }
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        let g1 = Guess::new(200);
        println!("haha {}", g1.value);
    }
}