
fn main() {
    let a: [i32;4] = [1,2,3,4]; 
    let b = &a[..=2]; 
    println!("a: {:?}\nb: {:?}",a, b);
 
    let my_bool = true;
    let otherbool = my_bool;
 
    println!("My Bool: {}\nAnother Bool: {}", my_bool, otherbool);
     
    let my_name =  "Victor";
    let my_name_again = my_name;
 
    println!("My name: {}\nMy name again: {}", my_name, my_name_again);
 
    //As we can see, all primitive types that are stored on the Stack Memory
    //It's possible to re-assigning, and coninues to use the variable.
    //In the cases above, if the type of `my_name` was a String type
    //the compiler would throw an error, because we'd have moved the variable
    //of `my_name` to `my_name_again`. This will make the my_name unsuable.
    //This happens with all data stored on Heap Memory!!


    let mut c = vec![1, 2, 3];
    {
      let _d = &mut c;  //  &mut borrow of `a` starts here
      // any other code
    }                  //  &mut borrow of `a` ends here
  
    println!("{:?}", c); // allow borrowing `a` as a shared borrow

    let mut e = vec![5,6,7];

    {
        let f = &mut e;
        f[0] = 1;
    }

    println!("\ne: {:?}",e);
    //This returns [1,6,7] and not [5,6,7]
    //This happens because we can't index a mutable
    //variable directly with [0], so the Rust infers it
    //should dereference it first. Basically a dereference
    //is happing, it's just implicit.

    

 }