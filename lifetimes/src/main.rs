//LIFETIMES IN RUST

// BORROW CHECKER ->
//
// The Rust Compiler has a borrow checker that compares scopes
// to determine whether all borrows are valid.

//    {
//        let r;                // ---------+-- 'a
//                              //          |
//        {                     //          |
//            let x = 5;        // -+-- 'b  |
//            r = &x;           //  |       |
//        }                     // -+       |
//                              //          |
//        println!("r: {}", r); //          |
//    }                         // ---------+
//
// The code above will not runs correctly, then we'll receive a compile
// error.
//
//
//Here, we’ve annotated the lifetime of r with 'a and the lifetime of x with 'b.
//As you can see, the inner 'b block is much smaller than the outer 'a lifetime block.
//At compile time, Rust compares the size of the two lifetimes and sees that r has a 
//lifetime of 'a but that it refers to memory with a lifetime of 'b. The program is 
//rejected because 'b is shorter than 'a: the subject of the reference doesn’t live as 
//long as the reference.
//
//fn main() {
//    {
//        let x = 5;            // ----------+-- 'b
//                              //           |
//        let r = &x;           // --+-- 'a  |
//                              //   |       |
//        println!("r: {}", r); //   |       |
//                              // --+       |
//    }                         // ----------+
//}
//
//
//Here, x has the lifetime 'b, which in this case is larger than 'a. This means r can 
//ref//erence x because Rust knows that the reference in r will always be valid while 
//x is val//id.

//Now that you know where the lifetimes of references are and how Rust analyzes 
//lifetimes to ensure references will always be valid, let’s explore generic lifetimes
//of parameters and return values in the context of functions.


//GENERIC LIFETIMES IN FUNCTIONS ->

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len(){
        x
    }else{
        y
    }
}

//Lifetimes are declared after the function name equal when
//we define a generic type. Then we attribute the generic lifetime
//to the parameters after the & signal, as you can see above in longest
//function signature. What happens is: The borrow checker will take
//the smaller lifetime between the lifetimes of the parameters. Then, 
//the generic lifetime will be equal this lifetime, and all references
//that are using a lifetime signature will have the same lifetime to 
//borrow checker. NOTE: The original lifetime doesn't change. The code 
//bellow show the use of longest function:

fn main() {
    /*
     EXAMPLE:

        let string1 = String::from("abcd"); -----> lifetime 'b
        let result; -----------------------------> lifetime 'b

        {
            let string2 = String::from("xyz"); --> lifetime 'a
            result = longest(string1.as_str(), --> lifetime 'a
            string2.as_str());

            //To the borrow checker the result will has the same
            //lifetime that string2 implements. Therefore, at the
            //end of this inner scope, the result and string2 will
            //be deallocated.Even though we create result outside
            //of this inner scope, we'll not be able to print result
                
        }
   
        println!("The longest string is {}", result);

        // To fix this error, code as bellow:
   */


    let string1 = String::from("abcd");
    
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("Result: {}", result);
    }
    
    //THINKING IN TERMS OF LIFETIMES -> 
    
    //The way in which you need to specify lifetime parameters depends on what your
    //function is doing. For example, if we changed the implementation of the longest
    //function to always return the first parameter rather than the longest string
    //slice, we wouldn’t need to specify a lifetime on the y parameter. The following 
    //code will compile:

    fn longest2<'a>(x: &'a str, _y: &str) -> &'a str {
        x
    }

    let string3 = String::from("efgh");
    let string4 = String::from("rstu");

    let result2 = longest2(string3.as_str(), string4.as_str());

    println!("Result2: {}", result2);

    //When returning a reference from a function, the lifetime parameter for the
    //return type needs to match the lifetime parameter for one of the parameters.
    //If the reference returned does not refer to one of the parameters, it must 
    //refer to a value created within this function, which would be a dangling
    //reference because the value will go out of scope at the end of the function. 
    //Consider this attempted implementation of the longest function that won’t compile:
    
    /* 
     * fn longest<'a>(x: &str, y: &str) -> &'a str {
     *      let result = String::from("really long string");
     *      result.as_str()
     * }
     *
     * */
    
    //Here, even though we’ve specified a lifetime parameter 'a for the return type,
    //this implementation will fail to compile because the return value lifetime is not
    //related to the lifetime of the parameters at all

    //NOTE: Ultimately, lifetime syntax is about connecting the lifetimes of various
    //parameters and return values of functions. Once they’re connected, Rust has enough
    //information to allow memory-safe operations and disallow operations that would
    //create dangling pointers or otherwise violate memory safety



}

