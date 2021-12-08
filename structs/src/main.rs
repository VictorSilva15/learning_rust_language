fn main() {

    //Using User struct:
    let mut user1 = User{
        email: String::from("victor470hugo@gmail.com"),
        username: String::from("VictorSilva15"),
        active: false,
        sign_in_count: 1,
    };
    
    println!("email: {}", user1.email);

    user1.email = String::from("victor.silvaPC@hotmail.com");

    println!("new email: {}", user1.email);
    
    //Calling build_user function
    let user2 = build_user("victor.silva898@etec.sp.gov.br", "Victor Hugo");

    println!("User2's Email: {}", user2.email);
    
    //Creating Instances from other instances with struct syntax:
    
    let user3: User = User {
        active: user2.active,
        email: String::from("victor888hugo@gmail.com"),
        username: String::from("Victin"),
        sign_in_count: user2.sign_in_count,
    };

    println!("User 3 active: {}", user3.active);


    //The syntax .. specifies that the remaining fields not explicitly 
    //set should have the same value as the fields in the given instance.

    let user4 = User {
        username: String::from("Hugo do corre"),
        ..user1
    };

    println!("\nThe User4 use the user1 datas:\nuser1.email: {}
             \nuser1.active: {}
             \nuser1.sign_in_count: {}", user4.email, user4.active, user4.sign_in_count);
    //We can't use user1.email and user1.username any more, because we have moved the values
    //into user4. We have moved only active and sign_in_count field, we'd can, since 
    //active and sign_in_count are type that makes copy in heap. The string is other situation
    //println!("User1 Email: {}", user1.email); <- This will throw a compiler error!

    
    //Calling Tuple Structs:
    
    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    println!("Black: {} {} {}", black.0, black.1, black.2);


    





}

//Creating a function that returns a value of type User (Strut)
fn build_user(email: &str, username: &str) -> User {
    User {
        email: email.to_string(),
        username: username.to_string(),
        active: true,
        sign_in_count: 1,
    }
}
//Struct called User that contains email, username, active and sign_in_count fields
struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}

//Creating a Tuple Struct:

struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

//The instances of the Structs above will be different types, because they'll be instances
//of different tuple structs. Each struct you define has its own type, even though the 
//fields within the struct have the same types. For example, a function that takes a 
//parameter of type Color cannot take a Point as an argument, even though both types are 
//made up of three i32 values. Otherwise, tuple struct instances behave like tuples: 
//you can destructure them into their individual pieces, you can use a . followed by 
//the index to access an individual value, and so on





