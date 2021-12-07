//References are used to refer to some value without taking
//ownership of it

//Rules to references:
//
//At any time, you can have either one mutable references, or any number of immutable references
//References must always be valid

//Using references:
fn main() {
    let mut s = String::from("hello");

    let mut r1 = &s; // no problem
    let r2 = &s; // no problem

    println!("{} and {}", r1, r2);
    //variables r1 and r2 will not be used after this point
    
    r1 = &String::from("Hello World");

    let r3 = &mut s; //no problem
    println!("{}", r3);

}
