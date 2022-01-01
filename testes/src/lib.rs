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

pub fn add_two(a: i32) -> i32 {
    a + 2
}

//All primitive types and most of the standard library types
//implement the PartialEq and Debug traits. To Structs and Enums
//that we create, we need implement, using: `#[derive(Debug)]`
//and #[derive(PartialEq)] or these traits together as show bellow:

#[derive(Debug, PartialEq)]
pub struct Square {
    size: u32
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
    //will checks if the arguments are differents.
    //The assert_ne!() macro is useful for cases when we're not sure what
    //a value will be, but we know what the value definitely won't be if our
    //code is functioning as we intend.

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_add_two_but_its_different(){
        assert_ne!(5, add_two(2));
    }

    //In some languages and test frameworks, the parameters to the functions
    //that assert two values are equal are called expected and actual,
    //and the other in which we specify the matters. However, in Rust, they're
    //called left and right, and the order in wich we specify the value,
    //we  expect and the value that the code under test produces does'nt matter.

    //When the assertions fail, these macros print their arguments using debug
    //formatting, which means the values being compared must implement the
    //PartialEq and Debug traits.

    //NOTE: All the primitive types and most of the standard library types
    //implement these traits. For Structs and Enums that you define, you'll
    //need to implement PartialEq to assert that the values of those types are 
    //Equal. You'll need to implement Debug to print the values when the assertion
    //fails.

    #[test]
    fn verify_structs_partialeq() {
        let square1 = Square {
            size: 12,
        };

        let square2 = Square {
            size: 12,
        };

        assert_eq!(square1, square2);
    }
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