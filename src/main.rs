use std::error::Error;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem building config: {}", err);
        std::process::exit(1);
    });

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
struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        };
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = std::env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
