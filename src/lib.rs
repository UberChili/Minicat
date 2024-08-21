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

pub fn cat(files: Vec<String>) -> String {
    let mut result: String = String::new();
    for file in files {
        if let Ok(contents) = fs::read_to_string(&file) {
            result += &contents;
        } else {
            eprintln!("Unable to read file {file}");
        }
    }
    result
}

//pub fn cat_with_numlines(files: Vec<String>) -> String {
//
//}

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

        assert_eq!(content_concatenated, cat(filenames));
    }

    #[test]
    fn with_numline() {
        let filenames = vec![
            "1.txt".to_string(),
            "2.txt".to_string(),
            "3.txt".to_string(),
        ];
        let content_concatenated = "1  one\n2  two\n3  three";
        assert_eq!(content_concatenated, cat_with_numlines(filenames));
    }
}
