#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
    
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width
            && self.height > rectangle.height
    }

    //Creating a associated function:
    

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

//Methods can have the same name that properties of a
//struct. we use &self instead of rectangule: &Rectangule.
//The &self is actually short for self:&Self. Within an impl
//block, the type Self is an alias for the type that the impl
//block is for. Methods must have a parameter named self of type
//Self for their first parameter, so Rust lets you abbreviate this
//with only the name self in the frist parameter spot. Note that
//we still need to use & in front of the self shorthand to
//indicate this method borrows the Self instance, just as we did
//in rectangle: &Rectangle. Methods can take ownership of self, 
//borrow self immutably as we've done here, or borrow self mutably
//just as they can any other parameter.
//
//We've chosen &self here for the same reason we used &Rectangle
//in the function version: we don't want to take ownership, and
//just want to read the data in the struct, no write to it. If
//we wanted to change the instance that we'be called the method
//on as part of what the method does, we'd use &mut self as the
//first parameter. Having a method taht takes ownership of
//instance by using just slef as the first paramenter is rare;
//this tchnique is usually used when the method transforms self
//into something else and you wnat to prevent the caller from
//using the original instance after the transformation.
//
//
//The main benefit of using methods instead of functions,
//in addition to using method syntax and not having to repeat
//the type of self in every method's signature, is for
//organization. We have put all the things we can do with an
//instance of a type in one impl block rather than making future
//users of our code search for capabilities of Rectangle in
//various places in the library we provide.
//
//
//we can use a field within a method of the same name for any
//purpose. In main, when we follow rect1.width with parentheses,
//Rust knows we mean the method width. When we don’t use 
//parentheses, Rust knows we mean the field width.
//
//Often, but not always, methods with the same name as a field 
//will be defined to only return the value in the field and do 
//nothing else. Methods like this are called getters, and Rust
//does not implement them automatically for struct fields as 
//some other languages do. Getters are useful because you can
//make the field private but the method public and thus enable
//read-only access to that field as part of the type’s public 
//API. We will be discussing what public and private are and 
//how to designate a field or method as public or private in 
//Chapter 7.

fn main() {
    let rect1 = Rectangle {
        width:30,
        height:50,    
    };
    
    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    println!("width > 0 : {}", rect1.width());
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 =  Rectangle {
        width: 60,
        height: 45,
    };

    println!("can rect1 hold rect2? -> {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? -> {}", rect1.can_hold(&rect3));

    //calling a associated function of Rectangle:

    let my_square = Rectangle::square(25);

    //We can also use println!("my_square: {:#?}", my_square);
    //but we'll  use dbg! macro syntax to print the occurence line
    //and structure together
    dbg!(my_square);


    //OBS:
    //:: syntax is used for both associated functions and namespaces
    //created by modules. We'll discuss modules in Chapter 7.


}


//In C and C++, two different operators are used for calling 
//methods: you use . if you’re calling a method on the object 
//directly and -> if you’re calling the method on a pointer to 
//the object and need to dereference the pointer first. In other 
//words, if object is a pointer, object->something() is similar
//to (*object).something().

//Rust doesn’t have an equivalent to the -> operator; instead,
//Rust has a feature called automatic referencing and 
//dereferencing. Calling methods is one of the few places in Rust
//that has this behavior.

//Here’s how it works: when you call a method with 
//object.something(), Rust automatically adds in &, &mut, or * 
//so object matches the signature of the method. In other words,
//the following are the same:
//
//
//p1.distance(&p2);
//(&p1).distance(&p2);
//
//
//The first one looks much cleaner. This automatic referencing 
//behavior works because methods have a clear receiver—the type 
//of self. Given the receiver and name of a method, Rust can 
//figure out definitively whether the method is reading (&self), 
//mutating (&mut self), or consuming (self). The fact that Rust
//makes borrowing implicit for method receivers is a big part of
//making ownership ergonomic in practice.
//


