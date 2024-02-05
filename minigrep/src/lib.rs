use std::env;
use std::error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(config.file_path)
        .expect("Could not read file");

    println!("{contents}");

    return Ok(());
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(args: env::Args) -> Result<Config, String> {
        let args: Vec<_> = args.collect();
        if args.len() < 3 {
            return Err(String::from("not enough arguemnts"));
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        return Ok(Self { query, file_path })
    }
}
