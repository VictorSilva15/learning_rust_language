fn main() {
  //GENERICS
  //
  //Sometimes we want write a function or data type that works
  //with different types of data. In rust we can do it using generics
  //
  //The concept is simple. Instead of declaring a specific data type
  //we use an uppercase letter(or PascalCase indentifier). Ex:
  //instead of x: u8; we use x: T. but we have to inform to the compiler
  //that T is a generic type (can be any type) by adding <T>at first:

    // Generic Functions
    
    fn _take_anything<T>(_x: T) {
        //x has type T, T is a generic type
    }

    fn _take_two_of_the_same_thing<T>(_x: T, _y: T){
        //Both x and y has the same type 
    }

    fn _takes_two_things<T,U>(_x:T, _y:U) {
        //Multiple types
    }

    //Generalizing structs
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }
        
    let point_a = Point{x: 0, y: 0}; // T is a int type
    let point_b = Point {x: 0.0, y: 0.0}; //T is a float type
    println!("\npoint_a.x: {} (i32)\npoint_b.y: {} (f64)\n", point_a.x, point_b.y);
    //Generalizing enums

    enum _Option<T>{
        Some(T),
        None,
    }
    enum _Result<T>{
        Ok(T),
        Err(T),
    }

    //Above Option and Result types are kind of special generic
    //types which are already defined in Rust's standard library
    //
    // - An Option<T> value can have either Some value or no value/None.
    // - A Result<T> value can represent either sucess/OK or failure/Err.
    
    //We use Option<T> when we don't know if the return will be the right.
    //so to prevent erros, we use Option<T> that returns either Some(T) value
    //or None. The same happens with Result
    
    //EXAMPLE WITH OPTION:
            
    let my_name = Phrase(String::from("I love Yuriko"));
    let number_of_words = my_name.number_of_words();
    
    match number_of_words {
        Some(words) => println!("Number of words: {}", words),
        None => println!("There're no words"),
    }

}

#[derive(Debug)]
struct Phrase(String);

impl Phrase {
    fn number_of_words(&self) -> Option<u32> {
        
        let words = self.0.trim()
            .split_whitespace()
            .count() as u32;
            
        match words {
            0 => None,
            1.. => Some(words)
        }
    }
}
