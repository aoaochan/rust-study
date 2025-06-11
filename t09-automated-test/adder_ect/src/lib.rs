pub fn add_two(a: usize) -> usize { a + 2 }
#[cfg(test)] 
pub mod test {
    fn internal_adder(left: usize, right: usize) -> usize { left + right }
    
    #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
}

// #[cfg(test)] 
// mod test {
//     fn add(x: i32, y: i32) -> i32 { x + y }

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

// #[cfg(test)]
// mod test {
//     fn add(x: i32, y: i32) -> i32 { x + y }

//     #[test]
//     fn it_work() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }

//     #[test]
//     #[ignore]
//     fn expensive_test() {
//         // ?
//     }
// }

// #[cfg(test)]
// mod test {
//     fn add_two(a: usize) -> usize { a + 2 }

//     #[test]
//     fn add_two_and_two() {
//         let result = add_two(2);
//         assert_eq!(result, 4);
//     }

//     #[test]
//     fn add_three_and_two() {
//         let result = add_two(3);
//         assert_eq!(result, 5);
//     }

//     #[test]
//     fn one_hundred() {
//         let result = add_two(100);
//         assert_eq!(result, 102);
//     }
// }

// #[cfg(test)]
// mod test {
//     fn prints_and_returns_10(a: i32) -> i32 {
//         println!("I got value {a}");
//         10
//     }

//     #[test]
//     fn this_test_will_pass() {
//         let val = prints_and_returns_10(4);
//         assert_eq!(val, 10);
//     }

//     #[test]
//     fn this_test_will_fail() {
//         let val = prints_and_returns_10(8);
//         assert_eq!(val, 5);
//     }
// }