fn main() {
   //Rust uses a memory managed through a system of 
   //onwership with a set of rules that the compiler 
   //checks at compile time.

    //Stack is the memory where variables and function calls is
    //allocated. It's like a plate stack, where when we use a plate
    //we put this one in the top of the stack. So when we want take
    //a plate, we picked up of the top. The same happens with the stack
    //memory. Exists 3 rules for ownership system:
    //
    // 1 - All values in rust has a variable that's called its owner
    // 2 - There can only be one owner at a time
    // 3 - WHen the onwer goes out of scope, the value will be dropped
    //
    //Known size types are stored in stack memory, but there's a type that
    //can have a unknown size, and then, this is stored diretly on the heap
    //memory
    //
    //
    //Heap memory is a large memory, otherwise a little slow than stack 
    //memory, where we can store some values, and then make a reference
    //of the pointer. This pointer is stored on the stack memory.
    //
    //
    //A type that's is stored on the heap is the String. There's a type
    //of string literals called str, the difference between String and str
    //is: 
    //
    //String is growable, heap-allocated data structure whereas str is an
    //immutable fixed-length string somewhere is memory
    //
    //
    

    //String type
    let string_01 = String::from("hello");

    //The double colon (::) is an operator that allow us to 
    //namespace this particular 'from' function under the
    //String type rather than using some sorte of name like
    //'string_from'.
    
    println!("String_01: {}", string_01);

    //This kind of string can be mutated:
    

    let mut string_02 = String::from("Hello_again");

    string_02.push_str(", world");

    println!("{}", string_02);

    
    //The memory is cleaned automatically whe the scope finished:

    {
        //string_03 is valid from this point forward
        let string_03 = String::from("I'm inside a new scope");
        println!("String_03: {}", string_03);
        //do stuff with string_03
    }//Here the scope finished, and the string_03 is deallocated


    //Moving with Integers:
    

    let x = 5;
    let y = x;


    println!("{} and {}", x, y);


    //The code above is allowed, because we're working with known size type, in this case
    //integers. So, we can get a 'copy' from x to y. But, in Rust we don't call this of 'copy'
    //but, 'move'. Here we moved the x value to y. But both the variables are valid
    

    //The same not happend with Strings  Type:


    let string_04 = String::from("from string_04 to string_05");

    let string_05 = string_04;
    
    println!("{}", string_05);

    //If we try show string_04, the compiler will throw an error, because
    //here we move string_04 to string_05. So string_04 is invalid. To fix this
    //we can use .clone(), to have a copy/clone of this variable:




    let string_06 = String::from("string_06 and string_07 are valid using clone method");
    let string_07 = string_06.clone();

    println!("\n{}\n{}", string_06, string_07);


    //Now will make an example of code that shows how the ownership rules works:
    

    fn example01(){
        let s1 = String::from("hello"); //s1 comes into scope

        takes_ownership(s1); // s's value moves into the function ....
                            // ... and so is no longer valid here

        let n1 = 5;         //x comes into scope

        makes_copy(n1);      //x would move into the function,
                            //but i32 is a copy, so it's okay to still
                            //use x afterward
    }// Here, x goes out of scope, then s. Because s's value was moved, nothing
    //special happens.

    fn  takes_ownership(some_string: String) {
        println!("{}", some_string);
    }

    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{}", some_integer);
    }// Here, some_integer goes out of scope

        
    example01();


    //Creating functions with returns



    fn example02(){
        let s1 = gives_ownership();         //gives_ownership moves its return
                                            //value into s1

        let s2 = String::from("hello");     //s2 comes into scope


        let s3 = takes_and_gives_back(s2);  //s2 is moved into 
                                            //takes_and_gives_back, which also
                                            //moves its return value into s3
        println!("Example02: {}, {}, {}", s1,s2,s3);

    }//Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens.
    //s1 goes out of scope and is dropped.


    fn gives_ownership() -> String {                //gives_ownership will move its
                                                    //return value into the function that
                                                    //calls it
        let some_string = String::from("yours");    //some_string comes into scope
    
        some_string //some_string is returned and moves out to the calling function
    }

    fn takes_and_gives_back(a_string: String) -> String{ //a_string comes into scope
        a_string // a_string is returned and moves out to the calling function
    }











        

}
