//Tests are important to virify if a piece of code
//are working well. Generally we want test something
//or figure out some bug, and the test can be useful to
//it. In Rust we can make tests creating a module called
//tests and put above the attribute #[cfg(test)] to configuration.
//Into the tests module, we define tests functions, that we need
//to specify to Rust that the function is a test with the #[test]
//attribute. With the test functions we can create code and
//call some macros that the Rust provice, between others that
//we'll see know:

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}



#[cfg(test)]
mod tests {

    //We use the syntax bellow to bring all code of the outer scope
    // in thes scope of tests module;
    use super::*;

    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    //The test bellow should panic, since we define the panic! macro into it
    #[test]
    #[should_panic]
    fn another() {
        panic!("Make this test fail");
    }


    //Checking results with the assert! Macro
    //The assert! macro, provided for standard library, is useful
    //when you want  to ensure that some  condition in a test evaluates to true.
    //This macro receive an argument that checks if is true or false. If true
    //the macro does nothing, else, it calls panic macro, which causes the 
    //test to fail.

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        //Using assert! macro:

        assert!(larger.can_hold(&smaller));
    }

    //The versio bellow will return false, so, we add  the ! expression
    //to convert the false to true, and make the code pass:

    #[test]
    fn smaller_cannot_hold_larger(){
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    //We've more two macro functions: The assert_eq!() and
    //the assert_ne!(). Basically assert_eq!() will compares
    //whether the arguments passed into it are equal. If the
    //arguments are equal, it does nothing, else it calls panic
    //macro. The same happens with assert_ne!(), however, assert_ne!()
    //will checks ....
}



//We can add as many additional test functions and
//as many test modules as we want! See:

#[cfg(test)]
mod tests2 {
    use std::io::Error;
    #[test]
    fn learning_test() {
        let result: Result<i32, Error> = Ok(20);
        assert!(result.is_ok());
    }
}