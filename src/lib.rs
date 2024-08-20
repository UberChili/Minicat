use std::{error::Error, fs};

pub struct Config {
    pub files: Vec<String>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let mut result = vec![];
        let args = &args[1..];
        for arg in args {
            result.push(arg.clone());
        }

        let files = result;
        Ok(Config { files })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    for file in config.files {
        let contents = fs::read_to_string(file)?;
        print!("{contents}");
    }

    Ok(())
}
