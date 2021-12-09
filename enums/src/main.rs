//Enum is a form that create possibilities of variants. For example:
//With Struct, we can define fields of different types, define methods,
//associated functions and so on. But, when we make a instance of a 
//struct, we must fill all fields of the struct. And it's impossible
//to define either one or another properties.
//
//With enums, we can literally enumerate all the Options, and then,
//assign the value to our variable:
#[derive(Debug)]
enum Message {
    Quit,
    Move {x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

//We can define methods to enumerations, using impl syntax, like
//the struct use. 

impl Message {
    fn call(&self){
        //..code that will execute into the variant choised
        dbg!(self);
    }
}

//Option standart library:
//
//The Option type is used in many places because it encodes the very 
//common scenario in which a value could be something or it could be 
//nothing. Expressing this concept in terms of the type system means 
//the compiler can check whether you’ve handled all the cases you should 
//be handling; this functionality can prevent bugs that are extremely
//common in other programming languages.
//
//Programming language design is often thought of in terms of which 
//features you include, but the features you exclude are important too. 
//Rust doesn’t have the null feature that many other languages have.
//Null is a value that means there is no value there. In languages with 
//null, variables can always be in one of two states: null or not-null.
//
//The problem with null values is that if you try to use a null value as
//a not-null value, you’ll get an error of some kind. Because this null 
//or not-null property is pervasive, it’s extremely easy to make this 
//kind of error.
//
//
//However, the concept that null is trying to express is still a useful
//one: a null is a value that is currently invalid or absent for some 
//reason.
//The problem isn’t really with the concept but with the particular 
//implementation. As such, Rust does not have nulls, but it does have an 
//enum that can encode the concept of a value being present or absent. 
//This enum is Option<T>, and it is defined by the standard library as 
//follows:

enum Option<T> {
    Some(T),
    None,
}
// It's not necessary define the enum Option in the scope, because
// the standart library includes it in prelude of code. So it's possible
// use Some(T) without define this enum, and it's not necessary use
// the Option:: syntax in the beginning of the expression



//The Option<T> enum is so useful that it’s even included in the prelude;
//you don’t need to bring it into scope explicitly. In addition, so are 
//its variants: you can use Some and None directly without the Option::
//prefix. The Option<T> enum is still just a regular enum, and Some(T)
//and None are still variants of type Option<T>.
//
//


fn main() {
    //to make a instance of a enum, we use:

    let m = Message::Write(String::from("Hello World"));

    //to call the methods we use:

    m.call();

    let some_number = Some(5);
    let some_string = Some("a string"); 
    
    //To get the value of some_number we can use match expression. But first of all
    //we need to know that there're some special values to match that help us a lot:
    //
    //-> other: Cover anything else that wasn't in the another arms.
    //-> _: When the value passed no matter, and we only want execute a function for example
    //so, we don't need catch te value, we just pass _ sintax.
    //-> (): This is passed after the =>, when we don't wanna do nothing, so, () is a special
    // unit tuple, that returns a special unit value.
    //
    let catch_value: u32 = match some_number {
        Some(i) => i,
        None => {
            println!("\nNot a number!\n");
            0
        },
    };

    println!("value catched: {}", catch_value);

    //ANOTHER EXAMPLE:

    let dice_roll = 7;

    match dice_roll {
        3 =>  you_lost(),
        7 => you_win(),
        other => {
            println!("Roll Again");  
        }
    }

    fn you_lost() {
        println!("You Lost!");
    }
    fn you_win(){
        println!("You won!");
    }


    //let absent_number: Option<i32> = None;

    //The type of some_number is Option<i32>. The type of some_string 
    //is Option<&str>, which is a different type. Rust can infer these
    //types because we’ve specified a value inside the Some variant. 
    //For absent_number, Rust requires us to annotate the overall Option
    //type: the compiler can’t infer the type that the corresponding 
    //Some variant will hold by looking only at a None value. Here, we
    //tell Rust that we mean for absent_number to be of type Option<i32>.
    //
    //
    //When we have a Some value, we know that a value is present and the
    //value is held within the Some. When we have a None value, in some 
    //sense, it means the same thing as null: we don’t have a valid value.
    //So why is having Option<T> any better than having null?
    //
    //In short, because Option<T> and T (where T can be any type) are 
    //different types, the compiler won’t let us use an Option<T> value 
    //as if it were definitely a valid value.
    //
    //Intense! In effect, this error message means that Rust doesn’t 
    //understand how to add an i8 and an Option<i8>, because they’re 
    //different types. When we have a value of a type like i8 in Rust,
    //the compiler will ensure that we always have a valid value. We can
    //proceed confidently without having to check for null before using 
    //that value. Only when we have an Option<i8> (or whatever type of 
    //value we’re working with) do we have to worry about possibly not 
    //having a value, and the compiler will make sure we handle that case
    //before using the value.
    //
    //In other words, you have to convert an Option<T> to a T before you 
    //can perform T operations with it. Generally, this helps catch one of
    //the most common issues with null: assuming that something isn’t null
    //when it actually is.
    //
    //In general, in order to use an Option<T> value, you want to have
    //code that will handle each variant. You want some code that will 
    //run only when you have a Some(T) value, and this code is allowed to
    //use the inner T. You want some other code to run if you have a None 
    //value, and that code doesn’t have a T value available. The match
    //expression is a control flow construct that does just this when used
    //with enums: it will run different code depending on which variant of
    //the enum it has, and that code can use the data inside the matching
    //value.
    
    understanding_match();
    
}

// Understanding match expression:

fn understanding_match(){
    let my_coin = Coin::Quarter(UsState::alaska);

    let my_coin_in_cents = my_coin.coin_in_cents();
    println!("my coin in cents is: {}", my_coin_in_cents);
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
    alabama,
    alaska,
}

impl Coin {
    fn coin_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                     println!("State quarter from {:?}!", state);
                     25
            },
        }
    }
}
