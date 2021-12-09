fn main() {
    //The if let syntax lets you combine if and let into a less verbose way
    //to handle values that match one pattern while ignoring the rest. 
    //Consider the program in Listing 6-6 that matches on an Option<u8> 
    //value in the config_max variable but only wants to execute code if 
    //the value is the Some variant

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    //If the value is Some, we want to print out the value in the Some variant,
    //which we do by binding the value to the variable max in the pattern. 
    //We don’t want to do anything with the None value. To satisfy the match 
    //expression, we have to add _ => () after processing just one variant, 
    //which is annoying boilerplate code to add.
    //
    //Instead, we could write this in a shorter way using if let. The following 
    //code behaves the same as the match in Listing 6-6:
    

    let config_max_iflet = Some(5u8);
    if let Some(max) = config_max_iflet {
        println!("The maximum is configured to be {}", max);
    }else{
        println!("anything to run...");
    }

    //The syntax if let takes a pattern and an expression separated by an equal sign. 
    //It works the same way as a match, where the expression is given to the match
    //and the pattern is its first arm. In this case, the pattern is Some(max), 
    //and the max binds to the value inside the Some. We can then use max in the
    //body of the if let block in the same way as we used max in the corresponding
    //match arm. The code in the if let block isn’t run if the value doesn’t match
    //the pattern.
    //
    //Using if let means less typing, less indentation, and less boilerplate code. 
    //However, you lose the exhaustive checking that match enforces. Choosing between 
    //match and if let depends on what you’re doing in your particular situation and 
    //whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.
    //
    //In other words, you can think of if let as syntax sugar for a match that runs code 
    //when the value matches one pattern and then ignores all other values.
    //
    //We can include an else with an if let. The block of code that goes with the else is the
    //same as the block of code that would go with the _ case in the match expression that
    //is equivalent to the if let and else.
    //
    //
    //If you have a situation in which your program has logic that is too verbose to express
    //using a match, remember that if let is in your Rust toolbox as well.
    //

    //SUMMARY
    

    //We’ve now covered how to use enums to create custom types that can be one of a set of enumerated
    //values. We’ve shown how the standard library’s Option<T> type helps you use the type system to
    //prevent errors. When enum values have data inside them, you can use match or if let to extract
    //and use those values, depending on how many cases you need to handle.
    //
    //Your Rust programs can now express concepts in your domain using structs and enums. Creating 
    //custom types to use in your API ensures type safety: the compiler will make certain your 
    //functions get only values of the type each function expects.
    //
    //In order to provide a well-organized API to your users that is straightforward to use and only
    //exposes exactly what your users will need, let’s now turn to Rust’s modules.  

}
