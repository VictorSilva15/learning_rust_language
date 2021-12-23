//RECOVERABLE ERROS WITH RESULT

use std::fs::File;
use std::io::{self, Read, ErrorKind};

pub mod recoverable {
    use super::*;

    pub fn recoverable_erros_with_result() {
        //Most errors aren’t serious enough to require the program 
        //to stop entirely. Sometimes, when a function fails, it’s for 
        //a reason that you can easily interpret and respond to. For example,
        //if you try to open a file and that operation fails because the file 
        //doesn’t exist, you might want to create the file instead of terminating 
        //the process

        //enum Result<T, E> {
        //    Ok(T),
        //    Err(E)
        //}   
        
        let f = File::open("hello.txt");

        //How do we know File::open returns a Result? We could look at the standard
        //library API documentation, or we could ask the compiler! If we give f a type 
        //annotation that we know is not the return type of the function and then try to 
        //compile the code, the compiler will tell us that the types don’t match. The error 
        //message will then tell us what the type of f is.

        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error=> {
                    panic!("Problem opening the file: {:?}", other_error);
                }
            },
        };

        println!("{:?}", f);   
        
        //The same could be written, without match expressions, that way:

        /*
         *  let f = File::open("hello.txt").unwrap_or_else(|error| {
         *      if error.kind == ErrorKind::NotFound {
         *          File.create("hello.txt").unwrap_or_else(|error| {
         *              panic!("Problem creating the file {:?}", error);
         *          }) 
         *      }else {
         *          panic!("Problem opening the file {:?}", error);    
         *      }
         *  }); 
         * 
         */


         //We can also use unwrap and expect methods, thar will try to return
         //the value Ok if the result be a success or a panic!() if occur an error.
         //The unwrap returns exactly the value within Ok(), and the panic!() message
         //is automatic. The expect method, will return the Result::Ok() itself, and
         //we have to define the message that will appear.

        let f2 = File::open("hello.txt").unwrap();

        print!("\nf2: {:?}", f2);

        let f3 = File::open("hello.txt").expect("Failed to open hello.txt");

        print!("\nf3: {:?}", f3);
    }

    pub fn propagating_error() {
        //When you’re writing a function whose implementation calls something that might fail, 
        //instead of handling the error within this function, you can return the error to the 
        //calling code so that it can decide what to do. This is known as propagating the error 
        //and gives more control to the calling code, where there might be more information or 
        //logic that dictates how the error should be handled than what you have available in the 
        //context of your code.


        fn read_username_from_file() -> Result<String, io::Error> {
            let f = File::open("hello.txt");

            let mut f = match f {
                Ok(file) => file,
                Err(e) => return Err(e),
            };

            let mut s = String::new();

            match f.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }

        }

        println!("\n\n--- Propagating Errors ---\n");
        
        println!("read_username_from_file: {:?}", read_username_from_file());



        //Exists  an operator `?` that does the same thing that the match expressions in
        //read_username_from_file function. Basically when we use ? in a return of Result
        //immediately we catch the value of Ok if the result was sucessed, or a Err 
        //if something else occured to use, we can make de same code above with this 
        //syntax


        fn read_username_from_file_2 () -> Result<String, io::Error> {
            let mut s = String::new();

            File::open("hello.txt")?.read_to_string(&mut s)?;
            
            Ok(s)
            

            // we can also write this way:

            /*
             * 
             * let mut f = File::open("hello.txt")?;
             * let mut s = String::new();
             * f.read_to_string(&mut s)?;
             * Ok(s);
             * 
             * 
            */
        }

        println!("\nread_username_from_file 2: {:?}\n", read_username_from_file_2());


        //We can only use ? expression in functions that returns a Result<T,E>, Option<Some, None>
        // or another type that implements std::io::Try because the ? syntax will return or a Ok 
        //value or a Err(e), To use ? into a function that doesn't return a Result we can do 
        //two things:

        //1°  One technique is to change the return type of your function to be Result<T, E> if 
        //you have no restrictions preventing that.

        //2° The other technique is to use a match or one of the Result<T, E> methods to handle the
        //Result<T, E> in whatever way is appropriate.

        //The main function is special, and there are restrictions on what its return type must be. 
        //One valid return type for main is (), and conveniently, another valid return type is
        //Result<T, E>, as shown here:

        /*
         * use std::error::Error;
         * use std::fs::File;
         * 
         *  fn main() -> Result<(), Box<dyn Error>> {
         *      let f = File::open("hello.txt")?;
         *      Ok(())
         *  }
         *  
         */

        //For now, you can read Box<dyn Error> to mean “any kind of error.” Using ? in a main function
        //with this return type is allowed.

    }
}
