use std::error::Error;

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
            result.push(arg);
        }

        let files = vec![];
        Ok(Config { files })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    for file in config.files {
        println!("{file}");
    }

    Ok(())
}
