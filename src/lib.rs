use std::{env, fs};
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("缺少查询参数"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("缺少查询的文件"),
        };

        let case_sensitive = match args.next() {
            Some(_) => false,
            None => true,
        };

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|p| p.contains(query))
        .collect()
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|p| { p.to_lowercase().contains(&query.to_lowercase()) })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn test_to_lowercase_search() {
        let query = "safe";
        let contents = "\
Rust:
sAfe, fast, productive.
Pick three.";
        assert_eq!(vec!["sAfe, fast, productive."], search_case_sensitive(query, contents));
    }
}