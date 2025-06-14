pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);
        // assert_eq!(result, 4);
        if result == 4 { Ok(()) }
        else { Err(String::from("two plus two doee not equal four")) }
    }
}
