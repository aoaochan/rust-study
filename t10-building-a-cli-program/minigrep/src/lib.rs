use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    // pub fn build(args: &[String]) -> Result<Config, &'static str> {
    pub fn build(
        mut args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a query string")
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a file path")
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        // if args.len() < 3 { return Err("not enough arguments"); }

        // let query = args[1].clone();
        // let file_path = args[2].clone();
        // let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    let result = if config.ignore_case { search_case_insensitive(&config.query, &contents) }
                        else { search(&config.query, &contents) };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()

    // let mut result = vec![];
    
    // for line in contents.lines() {
    //     if line.contains(query) { result.push(line); } 
    // }

    // result
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = vec![];

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    const CONTENTS: &'static str = "\
Rust:
safe, fast, productive.
Pick three.";

    #[test]
    fn case_sensitive() {
        let query = "duct";
        assert_eq!(vec!["safe, fast, productive."], search(query, CONTENTS));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        assert_eq!(
            vec!["Rust:", "Trust me"],
            search_case_insensitive(query, CONTENTS)
        );
    }
}