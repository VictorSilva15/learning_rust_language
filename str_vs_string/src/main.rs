//The main difference between str and String is
//String is growable, heap-allocated data structure whereas
//str is an immutable fixed-length string somewhere in memory

//String maintains a length and a capacity whereas a str only has
// a len() method. Example:

fn main() {
    let mut s = String::from("Hello rust!"); 

    //string
    println!("{}", s.capacity());
    s.push_str(" Here I come!");
    println!("{}", s.len());

    //str
    let s = "Hello, Rust!";
    println!("{}", s.len());
    //str has no capacity() method, just a len()

    example01();
    example02();

}

//&str
//You can only ever interact with str as a borrowed type aka &str. 
//This is called a string slice, an immutable view of a string. 
//This is the preferred way to pass strings around, as we shall see.

//&String
//This is a reference to a String, also called a borrowed type. 
//This is nothing more than a pointer which you can pass around 
//without giving up ownership. Turns out a &String can be coerced to a &str:

fn example01(){
    let s = String::from("Hello, rust!");
    foo(&s);
}

fn foo(s: &str){
    println!("{}", s);
}

//In the above example, foo() can take either string slices or 
//borrowed Strings, which is super convenient. As such, you almost 
//never need to deal with &Strings. The only real use case I can think of is 
//if you want to pass a mutable reference to a function that needs to 
//modify the string:

fn example02(){
    let mut s = String::from("Hello, rust!");
    foo2(&mut s);
}

fn foo2(s:&mut String){
    s.push_str("appeding foo..");
    println!("{}", s);
}

//Summary
//Prefer &str as a function parameter or if you want a read-only 
//view of a string; String when you want to own and mutate a string.
