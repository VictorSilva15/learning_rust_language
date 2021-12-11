//To import hashmap we use:
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    //to insert data into socres we use .insert :

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);


    //Just like vectors, hash maps store their data on the heap. 
    //This HashMap has keys of type String and values of type i32. 
    //Like vectors, hash maps are homogeneous: all of the keys must 
    //have the same type, and all of the values must have the same type.

    /*
    Another way of constructing a hash map is by using iterators and the 
    collect method on a vector of tuples, where each tuple consists of a 
    key and its value. We’ll be going into more detail about iterators and 
    their associated methods in the ”Processing a Series of Items with 
    Iterators” section of Chapter 13. The collect method gathers data into 
    a number of collection types, including HashMap. For example, if we had 
    the team names and initial scores in two separate vectors, we could use 
    the zip method to create an iterator of tuples where “Blue” is paired 
    with 10, and so forth. Then we could use the collect method to turn that 
    iterator of tuples into a hash map, as shown in Listing 8-21.
    */


    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores_2: HashMap<_,_> =
            teams.into_iter().zip(initial_scores.into_iter()).collect();
    
    /*
     * The type annotation HashMap<_, _> is needed here because it’s possible 
     * to collect into many different data structures and Rust doesn’t know 
     * which you want unless you specify. For the parameters for the key and 
     * value types, however, we use underscores, and Rust can infer the types 
     * that the hash map contains based on the types of the data in the vectors. 
     * In Listing 8-21, the key type will be String and the value type will be i32
    */

    /*
    * For types that implement the Copy trait, like i32, the values are copied 
    * into the hash map. For owned values like String, the values will be moved 
    * and the hash map will be the owner of those values, as demonstrated in 
    * Listing 8-22.
    */

    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    map.insert(field_name, field_value);
    //field_name and field_value are invalid at this point.


    //ACESSING VALUES IN HASHMAP

    let team_name = String::from("blue");
    let score = scores.get(&team_name);

    //The score will be a Some(&10) because the scores.get()
    //returns a Option<&V> so, to access the values, we can use
    //the syntax below:

    match score {
        Some(s) => println!("Blue's team score: {}", s),
        None => println!("No value"),
    }

    //Another way to get the key and value fields of a hashmap is
    //iterating:

    for (key, value) in &scores {
        println!("{}'s team score: {}", key, value);
    }

    //Updating values of a hashmap:

    //OVERWRITE values =>

    scores.insert(String::from("blue"), 25);
    scores.insert(String::from("yellow"), 42);
    scores.insert(String::from("blue"), 36);

    //We only need to set a new value putting the same
    //key name that the old.

    let score = scores.get(&String::from("blue"));

    match score {
        Some(s) => println!("Blue's team score: {}", s),
        None => println!("No value"),
    }

    //We can also use entry() method, that gets a parameter
    //and changes the value of a key depending if the key has
    //or not a value. The parameter is the key name that you want
    //to verify. Using entry() respective we can use the or_insert()
    //method in case of the key has no value assigned:

    let mut new_score = HashMap::new();

    new_score.insert(String::from("red"), 16);

    new_score.entry(String::from("green")).or_insert(25);
    new_score.entry(String::from("red")).or_insert(20);

    println!("{:?}", new_score);
    //The result will be {"green": 25, "red": 16}
    //because, as red key already has a value, the entry 
    //return an Enum called Entry that represents the value
    //that might or might not exist

    //The or_insert method on Entry is defined to return a mutable reference 
    //to the value for the corresponding Entry key if that key exists, and if not, 
    //inserts the parameter as the new value for this key and returns a mutable 
    //reference to the new value. This technique is much cleaner than writing the 
    //logic ourselves and, in addition, plays more nicely with the borrow checker.

    /*
    Another common use case for hash maps is to look up a key’s value and then
    update it based on the old value. For instance, Listing 8-26 shows code
    that counts how many times each word appears in some text. We use a hash
    map with the words as keys and increment the value to keep track of how
    many times we’ve seen that word. If it’s the first time we’ve seen a 
    word, we’ll first insert the value 0. 
    */

    let text = "hello world wonderful world";

    let mut new_map = HashMap::new();
    //split_whitespace() method returns the string splitted by
    //whitespaces

    for word in text.split_whitespace() {
        let count = new_map.entry(word).or_insert(0);
        *count += 1;
    }

    //This code will print {"world": 2, "hello": 1, "wonderful": 1}. 
    //The or_insert method actually returns a mutable reference (&mut V) 
    //to the value for this key. Here we store that mutable reference in 
    //the count variable, so in order to assign to that value, we must 
    //first dereference count using the asterisk (*). The mutable reference 
    //goes out of scope at the end of the for loop, so all of these changes 
    //are safe and allowed by the borrowing rules.

    


}
