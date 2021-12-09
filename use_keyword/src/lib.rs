//repetitive. For example, in Listing 7-7, whether we chose the 
//absolute or relative path to the add_to_waitlist function, every
//time we wanted to call add_to_waitlist we had to specify front_of_house
//and hosting too. Fortunately, there’s a way to simplify this process.
//We can bring a path into a scope once and then call the items in that
//path as if they’re local items with the use keyword.
//
//In Listing 7-11, we bring the crate::front_of_house::hosting module into 
//the scope of the eat_at_restaurant function so we only have to specify
//hosting::add_to_waitlist to call the add_to_waitlist function in
//eat_at_restaurant.

/*
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(){}
    }
}

use crate::front_of_house::hosting;
//we can call using `self` to refer to actual directory:
// use self::front_of_house::hosting;


//We can use:
//crate::front_of_house::hosting::add_to_waitlist;
//Thus, we can call the function directually in this scope.
//The problem is that, we can't to say if the function is
//a local function or not. It's can happen in bigger code.
//So... we specify  the module before the function that we 
//want.
//
//
//The same not happen with Struct, Enum, and other items.
//With these items we can specfy the full path to bring
//the items to the scope:

pub use std::collections::HashMap;

//When we bring a name into scope with the use keyword, the name available 
//in the new scope is private. To enable the code that calls our code to 
//refer to that name as if it had been defined in that code’s scope, we can 
//combine pub and use. This technique is called re-exporting because we’re
//bringing an item into scope but also making that item available for others
//to bring into their scope.

//let mut map = HashMap::new();
//map.insert(1,2)


pub fn eat_at_restaurant(){
    hosting::add_to_waitlist();
}

//Adding use and a path in a scope is similar to creating a symbolic link 
//in the filesystem. By adding use crate::front_of_house::hosting in the 
//crate root, hosting is now a valid name in that scope, just as though 
//the hosting module had been defined in the crate root. Paths brought
//into scope with use also check privacy, like any other paths.

//The exception to this idiom is if we’re bringing two items with the 
//same name into scope with use statements, because Rust doesn’t allow 
//that. Listing 7-15 shows how to bring two Result types into scope that
//have the same name but different parent modules and how to refer to them.

use std::fmt;
use std::io;

//We can call the io library changing its name with `as`:
// -> use std::fmt::Result;
// -> use std::io::Result as IoResult;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()>{
    // --snip--
}

//As you can see, using the parent modules distinguishes the two Result types.
//If instead we specified use std::fmt::Result and use std::io::Result, we’d
//have two Result types in the same scope and Rust wouldn’t know which one we
//meant when we used Result.




//If we’re using multiple items defined in the same crate or same module,
//listing each item on its own line can take up a lot of vertical space in our 
//files. For example, these two use statements we had in the Guessing Game in
//Listing 2-4 bring items from std into scope:
//
//
//use std::cmp::Ordering;
//use std::io;
//
//can be:
//
//
//use std::{cmp::Ordering, io};
//
//
//Instead, we can use nested paths to bring the same items into scope in one line.
//We do this by specifying the common part of the path, followed by two colons, 
//and then curly brackets around a list of the parts of the paths that differ,
//as shown in Listing 7-18.
//
//
//in case of:
//
//use std::io;
//use std::io:write;
//
//we can do:
//
//use std::io::{self, write};
//
//If we want to bring all public items defined in a path into scope, 
//we can specify that path followed by *, the glob operator:
//
//use std::collections::*;
//
//
//This use statement brings all public items defined in std::collections 
//into the current scope. Be careful when using the glob operator! Glob
//can make it harder to tell what names are in scope and where a name 
//used in your program was defined.
//
//The glob operator is often used when testing to bring everything under test 
//into the tests module; we’ll talk about that in the “How to Write Tests” 
//section in Chapter 11. The glob operator is also sometimes used as part of
//the prelude pattern: see the standard library documentation for more
//information on that pattern.
//
*/

mod front_of_house_2; // Will import the code of the file that has the same name of the module

pub use crate::front_of_house_2::hosting2;

pub fn eat_at_restaurant_2(){
    hosting2::add_to_waitlist();
}
