fn main() { 
    //Variables are immutable by default!

    //Mutable varibles
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
	
    //Using constants:

    const  THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("{}", THREE_HOURS_IN_SECONDS);

    //Shadowing

    let mut y = 10;
    
    {
        let y = y + 10;
	
        println!("Inside of scope y is: {}", y);    
 
    }
    y += 5;
    println!("Outside oif scope y is: {}", y);
}
