mod handle_erros;
use handle_erros::recoverable;

//ERROR HANDLING 
//
//errors can be two types, recoverable and unrecoverable erros.
//For recoverable errors, such as a file not found error. It's 
//reasonable to report the problem to the user and retry the operation.
//Unrecoverable erros are always symptoms of bugs, like trying to access
//a location beyond the end of an array.


//In some case we want stop/abort the program immediatelly, to do that, we need
//to use the panic!() macro with the abort property. It's simple, just go to the
//Cargo.toml and add `[profile.release]` and then `panic = 'abort'`


fn main() {
    let x = 12;

    if x < 10 {
        panic!("Error occured");
    }
    //The code above will print this:

    //thread 'main' panicked at 'Error occured', src\main.rs:19:9
    //note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace     
    //error: process didn't exit successfully: `target\debug\erros.exe` (exit code: 101)

    println!("x: {}", x);

    //To handle the error and get more details of what are happens, you can use:
    //`RUST_BACKTRACE=1` before the `cargo run` syntax to get a list of all the functions
    //called to program runs

    //RECOVERABLE ERROS:

    recoverable::recoverable_erros_with_result();
    recoverable::propagating_error();
}
