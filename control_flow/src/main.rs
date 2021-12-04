fn main() {
    let can_i_enter = valid(18);

    if can_i_enter {
        println!("You can enter");
    }else{
        println!("Acess denied!");
    }



    // LOOPS: Loop, While and For


    // Loop => 

    let mut count = 0;

    loop {
        if count == 10 {
            break;
        }
        else{
            println!("count = {}", count);
        }
        count += 1;
    }
    println!("You arrive in 10!");

    println!("\n--- Second Loop ---\n");

    // Loop with label =>
    let mut count_2 = 0;
    'counting_up: loop {
        println!("count_2 = {}", count_2);

        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count_2 == 2{
                break 'counting_up;
            }

            remaining -= 1;
        }  

        count_2 += 1;
    }

    println!("End count = {}", count_2);


    println!("\n--- Returning values from loops ---\n");

    //Returning Values from loops

    /*
    One of the uses of a loop is to retry an operation you know might fail, 
    such as checking whether a thread has completed its job. However, you 
    might need to pass the result of that operation to the rest of your code. 
    To do this, you can add the value you want returned after the break 
    expression you use to stop the loop; that value will be returned out 
    of the loop so you can use it, as shown here:
    */

    let mut counter = 0;


    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }

    };


    println!("The result is {}", result);

    println!("\n--- Using While loop ---\n");


    // While =>


    let mut n1 = 0;

    while n1 <= 10 {
        let res = n1 * 7;
        println!("{} x 7 = {}", n1, res);
        n1 += 1;
    }


    println!("\n--- Using For loop ---\n");

    // For => 


    let array = [10,20,30,40,50];

    for number in array {
        println!("The number is {}", number);
    }


    // Range and rev() =>


    println!("\n--- Range and rev ---\n");

    for nums in (1..4).rev() {
        println!("{}", nums);
    }

    //If we don't use the function rev(), it's not necessary use parentheses in the range 1..4;
    // rev() will reverse the sequence of the range (1..4 becomes 4..1)

}

fn valid(age:u8) -> bool{
   if age < 18 {
        false
        //I can use: return false instead of just false
        //But since the false is a expression, this sintax works fine
    }else{
        true
        // I can use return false instead of just true
        //But since the false is a expression, this sintax works fine
    }

    /*
    I can to assign values to a variable using if...else expression:println!

    let age = 21;

    let age = if age >= 18 {"You can enter"} else {"You canot enter"};

    println!("{}", age); // will be "You can enter"
    
    
    */

}