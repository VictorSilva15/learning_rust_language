mod strings;

use strings::manipulating_strings;


fn main() {


    //There're three type of collections:
    // -> Vectors
    // -> Strings
    // -> and HashMaps
    //
    // These types of collections are included
    // in Rust's standard library. There're a lot
    // of useful data structures called collections.
    //
    // These types are stored on the heap memory directualy
    // and are able to shrink and grow as the program runs.

    //Vect<T> known as a Vector

    //Vector allow you to store more than one value in a
    //single data structure that puts all the values
    //next to each other in heap memory. Vectos can only
    //store values of the same type. They are useful when
    //you have a list of items, such as the lines of text
    //in a file or the prices of items in a shopping cart.

    //Create a Vector:

    let _v1: Vec<i32> = Vec::new();

    //we specified the type that the vector will store
    //but we can define using only the values after
    //the signal '='.

    let _v2 = vec![1, 2, 3];
    //To do that we need to use the vec! macro.

    //Because we’ve given initial i32 values, Rust can
    //infer that the type of v2 is Vec<i32>, and the type
    //annotation isn’t necessary. Next, we’ll look at how
    //to modify a vector.

    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    // v3.pop(); removes and returns the last element!

    //Such as any variable, if we want to be able to change
    //values, we need to sue 'mut' keyword before the name
    //of variable, and with Vectors aren't different.

    //Like any other struct, a vector is freed when it goes
    //out of scope, as annotated below:

    {
        let _v4 = vec![1, 2, 3, 4];
    }

    //When the vector gets dropped, all of its contents are
    //also dropped, meaning those integers it holds will
    //be cleaned up. This may seem like a straightforward
    //point but can get a bit more complicated when you start
    //to introduce references to the elements of the vector.
    //Let’s tackle that next!

    //There are to ways to read elements of Vectors:
    //-> using index
    //-> using get() method

    let v5 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v5[2];
    println!("The third element is {}", third);

    match v5.get(20) {
        Some(third) => println!("The third element is {}", third),
        None => println!("The is no third element."),
    }

    //When the program has a valid reference, the borrow checker enforces
    //the ownership and borrowing rules (covered in Chapter 4) to ensure
    //this reference and any other references to the contents of the vector
    //remain valid. Recall the rule that states you can’t have mutable and
    //immutable references in the same scope. That rule applies in Listing
    //8-7, where we hold an immutable reference to the first element in a
    //vector and try to add an element to the end, which won’t work if we
    //also try to refer to that element later in the function

    let /*mut*/  v6 = vec![1,2,3,4,5];

    let first = &v6[0];

    //v6.push(6); Will throw an error!

    println!("The first element is {}", first);

    //Iteranting over the Values in a Vector

    let v7 = vec![100, 32, 57];

    for i in &v7 {
        println!("v7 = {}", i);
    }

    //Iteranting over mutable references to each
    //element in a mutable vector in order to make
    //changes to all the elements:

    let mut v8 = vec![100, 32, 57];

    for i in &mut v8 {
        *i += 50;
    }

    println!("{}", v8[2]);

    //To change the value that the mutable reference refers to,
    //we have to use the dereference operator (*) to get to the
    //value in i before we can use the += operator.

    //At the beginning of this chapter, we said that vectors can only
    //store values that are the same type. This can be inconvenient;
    //there are definitely use cases for needing to store a list of
    //items of different types. Fortunately, the variants of an enum
    //are defined under the same enum type, so when we need to store
    //elements of a different type in a vector, we can define and
    //use an enum!
    //
    //For example, say we want to get values from a row in a spreadsheet
    //in which some of the columns in the row contain integers, some
    //floating-point numbers, and some strings. We can define an enum whose
    //variants will hold the different value types, and then all the enum
    //variants will be considered the same type: that of the enum. Then we
    //can create a vector that holds that enum and so, ultimately, holds
    //different types.
    //

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    match &row[0] {
        SpreadsheetCell::Int(n) => println!("{}", n),
        SpreadsheetCell::Text(t) => println!("{}", t),
        SpreadsheetCell::Float(f) => println!("{}", f),
    }

    //Rust needs to know what types will be in the vector at
    //compile time so it knows exactly how much memory on the
    //heap will be needed to store each element. A secondary
    //advantage is that we can be explicit about what types are
    //allowed in this vector. If Rust allowed a vector to hold
    //any type, there would be a chance that one or more of the
    //types would cause errors with the operations performed on
    //the elements of the vector. Using an enum plus a match
    //expression means that Rust will ensure at compile time that
    //every possible case is handled, as discussed in Chapter 6.


    //UTF-8

    let mut my_name: String =  manipulating_strings::creating_strings("Victor");
    println!("name: {}", my_name);
    let some_string = " Hugo";
    my_name = manipulating_strings::updating_strings(my_name.as_str(), some_string);
    println!("new name: {}", my_name);

    println!("{}", some_string.trim());
    //The push_str method takes a string slice because we don’t necessarily 
    //want to take ownership of the parameter. Thus, we can to use the some_string
    //after use it in updating_string that contains a push_str method.

    //If the push_str method took ownership of s2, we wouldn’t be able to print its value 
    //on the last line. However, this code works as we’d expect!

    let my_slice_string = "lo";
    let my_slice_string = manipulating_strings::updating_character(my_slice_string, 'l');
    println!("my_slice_string: {}", my_slice_string);

    //we can also use + signal to concatenate strings:

    let s1 = String::from("Hello, ");
    let s2 = String::from("tic");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    println!("{}", s3);

    /*
     * The string s3 will contain Hello, world! as a result of this code. The reason 
     * s1 is no longer valid after the addition and the reason we used a reference to 
     * s2 has to do with the signature of the method that gets called when we use the + 
     * operator. The + operator uses the add method, whose signature looks something like this:
     * 
     * fn add(self, s: &str) -> String {}
     * 
     * This isn’t the exact signature that’s in the standard library: in the standard library, 
     * add is defined using generics. Here, we’re looking at the signature of add with concrete 
     * types substituted for the generic ones, which is what happens when we call this method 
     * with String values. We’ll discuss generics in Chapter 10. This signature gives us the clues 
     * we need to understand the tricky bits of the + operator. 
     * 
     * First, s2 has an &, meaning that we’re adding a reference of the second string to the first 
     * string because of the s parameter in the add function: we can only add a &str to a String; 
     * we can’t add two String values together. But wait—the type of &s2 is &String, not &str, 
     * as specified in the second parameter to add. 
     * 
     * The reason we’re able to use &s2 in the call to add is that the compiler can coerce the &String
     *  argument into a &str. When we call the add method, Rust uses a deref coercion, which here turns
     *  &s2 into &s2[..]. We’ll discuss deref coercion in more depth in Chapter 15. Because add does 
     * not take ownership of the s parameter, s2 will still be a valid String after this operation. 
     * Second, we can see in the signature that add takes ownership of self, because self does not 
     * have an &. This means s1 in Listing 8-18 will be moved into the add call and no longer be valid
     *  after that. So although let s3 = s1 + &s2; looks like it will copy both strings and create a 
     * new one, this statement actually takes ownership of s1, appends a copy of the contents of s2, 
     * and then returns ownership of the result. In other words, it looks like it’s making a lot of 
     * copies but isn’t; the implementation is more efficient than copying. 
     * 
     * If we need to concatenate multiple strings, the behavior of the + operator gets unwieldy:
    */

    let s4 = String::from("tic");
    let s5 = String::from("tec");
    let s6 = String::from("toe");

    let s7 = s4 + "-" + &s5 + "-" + &s6;

    println!("s7, using + signal to concatenate: {}", s7);

    //At this point, s will be tic-tac-toe. With all of the + and " 
    //characters, it’s difficult to see what’s going on. For more 
    //complicated string combining, we can use the format! macro:

    let s8 = format!("{}-{}-{}", s2,s5,s6);

    println!("s8, using format! macro: {}", s8);


    /*
     This code also sets s to tic-tac-toe. The format! macro works in the same way as println!, 
     but instead of printing the output to the screen, it returns a String with the contents. 
     The version of the code using format! is much easier to read, and the code generated by 
     the format! macro uses references so that this call doesn’t take ownership of any of its
     parameters.
    */

    //We can't take a character of a string using index, because the strings are stored
    //as vectors, where each caracter are allocated in a point, to get a value, we can iterate:

    let name = String::from("Victor");

    println!("\n");
    //using chars() to break the string apart 
    for c in name.chars() {
        println!("{}", c);
    }

    println!("\n");
    //using bytes() or as_bytes() to return each raw byte:

    for c in name.as_bytes(){
        println!("{}", c);
    }
    





}



