//Importing standard libraries
use std::{env, process};

use minigrep::Config;


fn main() {
    //Creating a Vector of strings by using args().collect()
    //args will return a Iterator and collect() will create the collection
    let args: Vec<String> = env::args().collect();

    // Getting arguments value from Config::new() and unwrap them
    // If the return be an error, so the closure are executed and finish the
    // process by calling process::exit(1). The '1' means that the code end with 
    // some error 
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("\nProblem parsing arguments: {}\n", err);
        process::exit(1)
    });

    println!("\nSearching for {}", config.query);
    println!("In file {}", config.filename);
    
    // run(config).unwrap_or_else(|error| {
    //     println!("\nApplication error: {}\n", error);
    //     process::exit(1)
    // });

    // or

    // match run(config) {
    //     Ok(()) => return,
    //     Err(error) => {
    //         println!("\nApplication error: {}\n", error);
    //         process::exit(1);
    //     }
    // }

    // or 

    if let Err(e) = minigrep::run(config) {
        println!("\nApplication error: {}\n", e);
        process::exit(1);
    }

    // In this case it's better use 'if let' statement because
    // we want only handle a specifc case, which is the Err().
    // we do that because, it's not necessary handle the success case (Ok())
    // since it's a unit type  [

}

