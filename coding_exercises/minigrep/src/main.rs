use minigrep::Config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Not enough arguments");
        std::process::exit(1);
    }

    let config = Config::new(&args);

    println!("Searching for {} in file {}", config.query, config.filename);

    minigrep::run(config);
}
