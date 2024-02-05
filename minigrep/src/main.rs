use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args = env::args();
    let config = Config::new(args)
        .unwrap_or_else(|err| {
            println!("{err}");
            process::exit(1);
        });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(err) = minigrep::run(config) {
        println!("{err}");
        process::exit(2);
    }
}
