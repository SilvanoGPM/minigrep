use std::error::Error;
use std::{env, fs};

use colored::*;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        const IGNORE_CASE_FLAG: &str = "--i";

        args.next();

        let query = match args.next() {
            Some(query) => query,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(path) => path,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case_flag = match args.next()  {
            Some(flag) if flag.to_lowercase() == IGNORE_CASE_FLAG => true,
            Some(_) => return Err("Invalid flag does not exist"),
            None => false,
        };

        let ignore_case_env = env::var("IGNORE_CASE").is_ok();

        let ignore_case = ignore_case_flag || ignore_case_env;

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for line in result {
        let colored_line = colorize_line(line, &config.query);

        println!("{colored_line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

fn colorize_line(line: &str, query: &str) -> String {
    line.split_whitespace()
        .map(|word| {
            if word.contains(query) {
                let colored_query = query.green().bold().to_string();
                let colored_rest = &word[query.len()..].red().bold().to_string();

                return colored_query + colored_rest;
            }

            word.red().bold().to_string()
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
            Rust:\n\
            safe, fast, productive.\n\
            Pick three.\n\
            Duct tape.";

        assert_eq!(search(query, content), vec!["safe, fast, productive."]);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
            Rust:\n\
            safe, fast, productive.\n\
            Pick three.\n\
            Trust me.\n";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, content)
        );
    }
}
