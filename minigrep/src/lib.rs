// Importing standard libraries 
use std::{fs, error::Error};

// Creating a strcuture with configuration fields related
pub struct Config {
    pub query: String,
    pub filename: String,
}

//function that receive reference to a vector of strings, and returns a tuple of &str
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {

        if args.len() < 3 {
           return Err("Not enough arguments")
        }


        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            query,
            filename,
        })
    }
}

// function that will find our file by passing config variable as a parameter. 
// It'll use fs::read_to_string to try to find it and print the result
// in case of nothing error
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{

    //Cacthing the content of a file in root directory by specifying the filename
    let contents = fs::read_to_string(config.filename)?;

    println!("\nWith text:\n{}\n", contents);

    Ok(())
}
