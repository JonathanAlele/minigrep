use std::error::Error;
use minigrep::Config;
fn main() {
    // get command line arguments
    let args: Vec<String> = std::env::args().collect();

    // create new config and set env variable to ignore case
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem building config: {}", err);
        std::process::exit(1);
    });

    // print results
    if let Err(err) = run(config) {
        eprintln!("Application error: {}", err);
        std::process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        minigrep::search_case_insensitive(&config.query, &contents)
    } else {
        minigrep::search_case_sensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}
