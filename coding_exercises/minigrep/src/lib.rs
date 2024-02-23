use std::fs;

pub struct Config {
    pub filename: String,
    pub query: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let filename = args[1].clone();
        let query = args[2].clone();

        Config { filename, query }
    }
}

pub fn run(config: Config) {
    let content =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    println!("With text:\n{}", &content[..30]);
    let text_found = search(&config.query, &content);
    for line in text_found {
        println!("{}", line);
    }
}

// lifetime for result is the same as the lifetime of content
// we have to explicitly specify the lifetime of the return value in this case
fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    content.lines().for_each(|line| {
        if line.contains(query) {
            result.push(line);
        }
    });
    result
}
