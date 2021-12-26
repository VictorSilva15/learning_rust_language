#![allow(dead_code, unused)]
//TRAITS
//
//Traits are kind of similar to interfaces in OOP languages.
//They are used to define the functionality a type must provide.
//Multiple traits can be implemented for a single type.
//
//But traits can also include default implementations of methods.
//Default methods can be overridden when implementing types

//Impls without traits

mod traits;

struct Player {
    first_name: String,
    last_name: String,
}

impl Player {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

// ⭐️ Implementation must appear in the same crate as the self type

// 💡 And also in Rust, new traits can be implemented for existing types even 
// for types like i8, f64 and etc.
// Same way existing traits can be implemented for new types you are creating.
// But we can not implement existing traits into existing types.

fn main() {
    let player_1 = Player {
        first_name: "Victor".to_string(),
        last_name: "Silva".to_string(),
    };

    println!("Player 01: {}", player_1.full_name());
    println!("\nPlayer 01 Reverse: {}", player_1.full_name_rev());
    player_1.baz();

    let player_2 = Player::new("Lincon", "Almeida");
    player_2.bar();

    let num: u8 = player_2.method();
    let greeting: String = player_2.method();

    println!("num: {}\ngreeting: {}", num, greeting);
    
    let kitty = Cat { sound: "Meow".to_string() };
    let the_bell = Bell { sound: "Ding Dong".to_string() };

    make_sound(&kitty);
    make_sound(&the_bell);


    traits::traits1::traits();
    traits::traits2::traits2();



}

//Impls & Traits without default methods

trait FullNameReverse{
    fn full_name_rev(&self) -> String;
}

impl FullNameReverse for Player {
    fn full_name_rev(&self) -> String {
       self.full_name().chars().rev().collect::<String>()
    }
}

//Impls, traits  & default methods

trait Foo {
    fn bar(&self);
    fn baz(&self) {
        println!("We called baz.");
    }
}

impl Foo for Player {
    fn bar (&self){
        println!("We called bar!");
        
    }
}

//Impl with Associated functions

impl Player{
    fn new(first_name: &str, last_name: &str) -> Player {
        Player {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }
}

//Traits with Generics

trait Trait<T> {
    fn method(&self) -> T;
}

impl Trait<u8> for Player {
    fn method(&self) -> u8 {
        16
    }
}

//Using generics I can make another implementation
//And pass a different type as parameter

impl Trait<String> for Player {
    fn method(&self) -> String {
        "Hello World".to_string()
    }
}

//Traits Inheritance


//I did not understand hot it works,
//I'm learning yet, later I'll add some 
//code about here.

//Traits Object

trait GetSound {
    fn get_sound(&self) -> String;
}

struct Cat {
    sound: String,
}

impl GetSound for Cat {
    fn get_sound(&self) -> String {
        self.sound.clone()
    }
}

struct Bell {
    sound: String,
}

impl GetSound for Bell {
    fn get_sound(&self) -> String {
        self.sound.clone()
    }
}

fn make_sound<T: GetSound>(t: &T) {
    println!("{}!", t.get_sound());

}

    
