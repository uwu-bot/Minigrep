use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);
    println!("Searching for: '{}'", config.query);
    println!("In file: {}", config.filename);

    let mut f = File::open(config.filename).expect("file not found.");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file.");
    println!("\n\n{}", contents);
}

struct Config {
    query: String,
    filaname: String,
}

parse_config(args: &[String]) -> Config &str) {
    let query  = &args[1];
    let filename = &args[2];

    Config { query, filename }
}
