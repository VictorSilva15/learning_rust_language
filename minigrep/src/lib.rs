// Importing standard libraries
use std::{error::Error, fs, env};

// Creating a strcuture with configuration fields related
#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

//function that receive reference to a vector of strings, and returns a Result
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive: bool;

        if args.len() == 4 {
            case_sensitive = if args[3] == String::from("1") {
                false
            }else{
                true
            }
        }else{
            case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        }
        

        Ok(Config { query, filename, case_sensitive })
    }
}

// function that will find our file by passing config variable as a parameter.
// It'll use fs::read_to_string to try to find it and print the result
// in case of nothing error
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //Cacthing the content of a file in root directory by specifying the filename
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    }else{
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
