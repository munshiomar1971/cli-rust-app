use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sens{
        search(&config.query, &contents)
    } else {
        search_case_insens(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sens: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguements");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sens = env::var("CASE_INSENS").is_err();

        Ok(Config { query, filename, case_sens })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insens<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sens() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
pick three.
        ";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insens() {
        let query = "rUsT";
        let contents = "\
Rust: 
safe, fast productive.
pick three. 
Trust me.
        ";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insens(query, contents)
        );
    }
}
