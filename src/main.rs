use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("Searching for {} in {}", config.query, config.file_path);

    let content =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{content}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Self {
        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Self { query, file_path }
    }
}