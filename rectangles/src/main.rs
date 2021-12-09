fn main() {
    
    explict_way();

    //Refactoring with tuples =>
    
    using_tuples();

    //Refactoring with Structs =>
    
    using_struct();
}

fn explict_way(){
    let width1 = 30;
    let height1 = 50;
    
    println!(
        "The area of the rectangle is {} square pixels",
        area(width1,height1)
    );

    fn area(width:u32, height:u32) -> u32 {
        width * height
    }
}

//The area function is supposed to calculate the area of
//one rectangle, but the function we wrote has two parameters.
//The parameters are related, but that’s not expressed anywhere 
//in our program. It would be more readable and more manageable 
//to group width and height together. 

//Using tuples:

fn using_tuples(){
    let rect1 = (30,50);

    println!(
        "The area of the rectangle is {} square pixels",
        area(rect1)
    );

    fn area(dimensions: (u32,u32)) -> u32 {
        dimensions.0 * dimensions.1
    }
}

//In one way, this program is better. Tuples let us add a bit
//of structure, and we’re now passing just one argument. 
//But in another way, this version is less clear: tuples 
//don’t name their elements, so our calculation has become 
//more confusing because we have to index into the parts of 
//the tuple.

//It doesn’t matter if we mix up width and height for the area 
//calculation, but if we want to draw the rectangle on the 
//screen, it would matter! We would have to keep in mind that 
//width is the tuple index 0 and height is the tuple index 1. 
//If someone else worked on this code, they would have to figure 
//this out and keep it in mind as well. It would be easy to 
//forget or mix up these values and cause errors, because we
//haven’t conveyed the meaning of our data in our code.


//Using structs: 

#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32,
}

fn using_struct(){
    let rect1 = Rectangle {
        width: 40,
        height: 60,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect1)
            );

    println!("rect1 is {:#?}", rect1);
    //The line above tells the compiler, that we want
    //print the rect1 struct. To do that, we need to
    //specify in println! structure that the content
    //which will be printed is for debug. So we put
    //{:?} instead of only {}. furthermore, we need
    //specify that the struct has a debug. so, it's
    //necessary put before the struct definition the sintax:
    //#[derive(Debug)]. If you want show the informations
    //formated, then you can use {:#?} rather than {:?}
    //
    //Making all this, it will be possible to run the code
    //withou any problems.
    //
    //There's another way to print out this structure, using
    //the macro dbg!();
    
    dbg!(rect1);
   

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

}
