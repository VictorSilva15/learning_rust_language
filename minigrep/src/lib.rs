// Importing standard libraries
use std::{error::Error, fs};

// Creating a strcuture with configuration fields related
#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

//function that receive reference to a vector of strings, and returns a Result
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// function that will find our file by passing config variable as a parameter.
// It'll use fs::read_to_string to try to find it and print the result
// in case of nothing error
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //Cacthing the content of a file in root directory by specifying the filename
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let mut results = Vec::new();

    //lines() method returns an iterator
    //The contains() method returns a Boolean response, which tells us
    //whether or not the string has the text passed to the contains() method
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick tree.";
            
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

}
