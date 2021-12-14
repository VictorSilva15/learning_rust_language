fn main() {
    println!("Hello, world!");

    another_function();
    function_with_parameters(5);
    print_labeled_measurement(10, 'h');


    let y = {
        let x = 5;
        x + 1
    };
    //Is the same that = let y = {let x = 5; x+1};
    println!("Y: {}", y);

    println!("\n --- Functiosn with returns --- \n");
    let five = five();

    println!("five() returns: {}", five);

    let module_of_17 = module(17);

    println!("The module of 17 is: {}", module_of_17);

    catching_a_function();
}

//Creating a new function

fn another_function(){
    println!("Another Function...");
}

//Creating a new function with parameters:
fn function_with_parameters(x: i32){
    println!("The value of x is {}", x);
}
//Creating a new function with more than one parameters
fn print_labeled_measurement(value:i32, unit_label: char){
    println!("The measurement is: {}{}", value, unit_label);
}

//Creating a function that return the final expression:
fn five() -> i32 {
    5
}

fn module(mut n1: i32) -> i32 {
    n1 = n1 % 2;
    n1
    // or it could be: return n1, both the way works
} 

// Variables can hold functions:
fn catching_a_function(){
    let p = five;
    println!("p is the function five() now: {}", p());
    //It'll return 5;
    println!("{}", five());
    //We can still use the five() function!
}

