fn main() {
    println!("\n--- VECTORS ---\n");

    //A vector is kind of a re-sizable array but all elements must be in the same type.
    //⭐️ It’s a generic type, written as Vec<T> . T can have any type, ex. The type of
    //a Vec of i32s is Vec<i32>. Also, Vectors always allocate their data in a dynamically allocated heap.
    
    //creating a vector

    let mut _a = Vec::<i32>::new(); //using new keyword
    let mut _b: Vec<char> = vec![];  // using vec! macro

    //Creating with data types

    let mut _a2: Vec<i32> = Vec::new();
    let mut _b2: Vec<i32> = vec![];
    let b3 = vec![1i32, 2, 3 ]; //sufixing 1st value with data type

    let mut _b4 = vec![1,2,3];
    let b5: Vec<i32> = vec![1,2,3];
    let b6 = vec![0;10];

    
    println!("b3: {:?}\nb5: {:?}\nb6: {:?}", b3, b5, b6);


    //Accessing and changing existing data

    let mut c = vec![5,4,3,2,1];

    c[0] = 1;
    c[1] = 2;

    println!("c: {:?}", c);

    //push and pop

    let mut d: Vec<i32> = Vec::new();

    d.push(1); // [1] ->  add an element to the end
    d.push(2); // [1,2]
    d.pop(); // [1] -> Remove an element from the end

   
    let array = ['a', 'b', 'c'];
    println!("\narray: {:?}", array);

    let mut  array = array.to_vec();
    println!("\narray to vector: {:?}", array);
    array.push('d');

    let collection: String = array.iter().collect();
    println!("\n array to collection: {:?}", collection);
    

    //Capacity and reallocation

    let mut e: Vec<i32> = Vec::with_capacity(10);
    println!("\nLength: {}, Capacity: {}", e.len(), e.capacity());
    //Length: 0, Capacity: 10;

    //These are all done without reallocating...
    for i in 0..10 {
        e.push(i);
    }

    //...but this may make the vector reallocate
    e.push(10);
    println!("e: {:?}", e);
    println!("\nNew Length: {}, new Capacity: {}", e.len(), e.capacity());


}
