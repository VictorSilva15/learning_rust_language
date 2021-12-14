fn main() {

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
    closures();
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
    let f = five;
    println!("f is the function five() now: {}", f());
    //It'll return 5;
    println!("{}", five());
    //We can still use the five() function!
    //
    //we can also define the type of the variable before
    //to assign it:

    let f2: fn(i32) -> i32 = module;

    println!("f2 will be a function equal module()\n
        f2(17) = {}", f2(17));
}

//CLOSURES

//Closures also known as anonymous functions or lambda functions
//the data types of arguments and returns are optional:

fn closures(){
    //traditional function:
    let x = 2;
    println!("get_square_value(x): {}", get_square_value(x));
    
    fn get_square_value(i: i32) -> i32 {
        i * i
    }

    //Using closures. With optional type declarations of
    //input and return types:

    let x2 = 6;
    let square = |i: i32| -> i32 {
        i * i
    };

    println!("closure square(x2): {}", square(x2));
    
    //Without type declarations of input and return types:

    let x3: f64 = 8.0;
    let square2 =  |i| i * i;
    //{} are optional for single-lined closures
    println!("square2(x3): {}", square2(x3));

    //Without type declarations we can take and return any type
    //depending of the argument passed, and work with them.
    
    //With optional type declarations; Creating and calling together

    let x_square = |i: i32| -> i32 {i * i}(x);
    //we can also write as follows:
    //let x_square = |i| -> i32 {i*i}(x);

    println!("x_square: {}", x_square);
}
