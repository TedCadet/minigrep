//! # minigrep
//! search a chain of characters in a text file and returns the lines
//! that contains at least one occurence. An option to ignore case is possible
//!
//! *Project from the official rust documentation*
//! # Examples
//! ```
//! cargo run -- to poem.txt true
//! ```

use std::env;
use std::error::Error;
use std::fs;

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // if args.len() < 3 {
        //     return Err("not enough arguments");
        // }

        // let query = args[1].clone();
        // let file_path = args[2].clone();
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didnt get a file path"),
        };

        let ignore_case: bool = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// pub fn eval_ignore_case(ignore_case: &str) -> bool {
//     match ignore_case {
//         "true" | "t" | "yes" | "y"  => true,
//         _other => false
//     }
// }
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

/// runs minigrep
/// # Errors
/// * Reading an non-existant file
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = match config.ignore_case {
        true => search_case_insensitive(&config.query, &contents),
        false => search(&config.query, &contents),
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

/// search a chain of characters in a text file and returns the lines
/// that contains at least one occurence (case sensitive)
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }

    // results
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// search a chain of characters in a text file and returns the lines
/// that contains at least one occurence (case insensitive)
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query_lowecase = query.to_lowercase();
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query_lowecase) {
    //         results.push(line);
    //     }
    // }

    // results
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query_lowecase))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = r"\
Rust: 
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = r"\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
