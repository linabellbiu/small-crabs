use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(config: &[String]) -> Result<Config, &str> {
        if config.len() < 3 {
            return Err("没有足够的参数");
        }
        Ok(Config {
            query: config[1].clone(),
            filename: config[2].clone(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}