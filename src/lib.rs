use std::{error::Error, fs};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Files
    pub files: Vec<String>,

    /// Show number lines
    #[arg(short, long)]
    pub line_num: bool,
}

pub struct Config {
    pub files: Vec<String>,
    pub line_num: bool,
}

impl Config {
    pub fn build(args: Args) -> Result<Config, &'static str> {
        let line_num: bool;
        if args.line_num {
            line_num = true;
        } else {
            line_num = false;
        }

        if args.files.len() < 2 {
            return Err("Not enough arguments");
        }

        let mut result = vec![];
        //let args = &args[1..];
        for arg in args.files {
            result.push(arg.clone());
        }

        let files = result;
        Ok(Config { files, line_num })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let results = if config.line_num {
        cat_with_numlines(config.files)
    } else {
        cat(config.files)
    };

    print!("{}", results.unwrap());
    Ok(())
}

/// Normal cat functionality
pub fn cat(files: Vec<String>) -> Result<String, Box<dyn Error>> {
    let mut result: String = String::new();
    for file in files {
        if let Ok(contents) = fs::read_to_string(&file) {
            result += &contents;
        } else {
            eprintln!("Unable to read file {file}.");
        }
    }
    Ok(result)
}

pub fn cat_with_numlines(files: Vec<String>) -> Result<String, Box<dyn Error>> {
    let mut result: String = String::new();
    let mut count = 1;
    for file in files {
        if let Ok(contents) = fs::read_to_string(&file) {
            result += &format!("     {}  {}", count, &contents);
            count += 1;
        } else {
            eprintln!("Unable to read file {file}.");
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normally() {
        let filenames = vec![
            "1.txt".to_string(),
            "2.txt".to_string(),
            "3.txt".to_string(),
        ];
        let content_concatenated = "one\ntwo\nthree\n";

        assert_eq!(content_concatenated, cat(filenames).unwrap());
    }

    #[test]
    fn with_numline() {
        let filenames = vec![
            "1.txt".to_string(),
            "2.txt".to_string(),
            "3.txt".to_string(),
        ];
        let content_concatenated = "     1  one\n     2  two\n     3  three\n";
        assert_eq!(content_concatenated, cat_with_numlines(filenames).unwrap());
    }
}
