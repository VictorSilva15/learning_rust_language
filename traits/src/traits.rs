pub mod  traits1 {

    pub trait Summary {
        //We can specify default behavior for some or all methods in a trait
        //instead of requiring implementations for all methods on every type
        //The code bellow, could also be writen this way:

        /*
        * fn summarize(&self) -> String;
        */

        //However, we define a default behavior to it, so, we can keep or override
        //each method's default behavior.

        fn summarize(&self) -> String {
            String::from("Read more...")
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    //We could also define the implementations without override the summarize method
    //using only the syntax as follows:

    /*
    * impl Summary for NewsArticle{}
    */


    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String{
            format!("{}: {}", self.username, self.content)
        }
    }

    pub fn traits() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people"
            ),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());

        /*

        *Note that because we defined the Summary trait and the NewsArticle and Tweet types in
        the same lib.rs in Listing 10-13, they’re all in the same scope. Let’s say this lib.rs 
        is for a crate we’ve called aggregator and someone else wants to use our crate’s 
        functionality to implement the Summary trait on a struct defined within their library’s 
        scope. They would need to bring the trait into their scope first. They would do so by 
        specifying use aggregator::Summary;, which then would enable them to implement Summary 
        for their type. The Summary trait would also need to be a public trait for another crate 
        to implement it, which it is because we put the pub keyword before trait in Listing 10-12.
        
        One restriction to note with trait implementations is that we can implement a trait on a type
        only if either the trait or the type is local to our crate. For example, we can implement 
        standard library traits like Display on a custom type like Tweet as part of our aggregator 
        crate functionality, because the type Tweet is local to our aggregator crate. We can also 
        implement Summary on Vec<T> in our aggregator crate, because the trait Summary is local to 
        our aggregator crate.
        
        But we can’t implement external traits on external types. For example, we can’t implement 
        the Display trait on Vec<T> within our aggregator crate, because Display and Vec<T> are 
        defined in the standard library and aren’t local to our aggregator crate. This restriction 
        is part of a property of programs called coherence, and more specifically the orphan rule, 
        so named because the parent type is not present. This rule ensures that other people’s code 
        can’t break your code and vice versa. Without the rule, two crates could implement the same 
        trait for the same type, and Rust wouldn’t know which implementation to use.

        */
    }

}


pub mod traits2 {
    //This mod is being used to do another versions and examples
    //of the all code above!


    /* 
    
    Creating a default implementation for summarize doesn’t require us to change anything about 
    the implementation of Summary on Tweet in Listing 10-13. The reason is that the syntax for 
    overriding a default implementation is the same as the syntax for implementing a trait method 
    that doesn’t have a default implementation. 
    
    Default implementations can call other methods in the same trait, even if those other methods 
    on’t have a default implementation. In this way, a trait can provide a lot of useful 
    functionality and only require implementors to specify a small part of it. For example, we 
    could define the Summary trait to have a summarize_author method whose implementation is 
    required, and then define a summarize method that has a default implementation that calls 
    the summarize_author method:
    
    */

    pub trait Summary  {

        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            format!("Read more from {} ...", self.summarize_author())  
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }   

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    /*

    After we define summarize_author, we can call summarize on instances of the Tweet struct, 
    and the default implementation of summarize will call the definition of summarize_author 
    that we’ve provided. Because we’ve implemented summarize_author, the Summary trait has 
    given us the behavior of the summarize method without requiring us to write any more code.
    
    */   

    pub fn traits2() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false
        };
    
        println!("1 new tweet: {}", tweet.summarize());
    }

}


pub mod traits3 {
    //TRAITS AS PARAMETERS

    //Now that you know how to define and implement traits, we can explore how to use traits 
    //to define functions that accept many different types.

    //For example, in Listing 10-13, we implemented the Summary trait on the NewsArticle and 
    //Tweet types. We can define a notify function that calls the summarize method on its item 
    //parameter, which is of some type that implements the Summary trait. To do this, we can use 
    //the impl Trait syntax, like this: 
    
    //Instead of a concrete type for the item parameter, we specify the impl keyword and the 
    //trait name. This parameter accepts any type that implements the specified trait. In the 
    //body of notify, we can call any methods on item that come from the Summary trait, such as 
    //summarize. We can call notify and pass in any instance of NewsArticle or Tweet. Code that 
    //calls the function with any other type, such as a String or an i32, won’t compile because 
    //those types don’t implement Summary.

    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }


    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        username: String,
        content: String,
        reply: bool,
        retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("@{}", self.username)
        }
    }

    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    //The same can be done as follows:

    pub fn notify2<T: Summary>(item: &T) {
        println!("notify2 - Breaking news! {}", item.summarize())
    }   

    //This longer form is equivalent to the example in the previous section but is more verbose. 
    //We place trait bounds with the declaration of the generic type parameter after a colon and 
    //inside angle brackets.
    
    //The impl Trait syntax is convenient and makes for more concise code in simple cases. 
    //The trait bound syntax can express more complexity in other cases. For example, we can have 
    //two parameters that implement Summary. Using the impl Trait syntax looks like this:

    /*
        pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
    */

    //If we wanted this function to allow item1 and item2 to have different types, using impl 
    //Trait would be appropriate (as long as both types implement Summary). If we wanted to force 
    //both parameters to have the same type, that’s only possible to express using a trait bound, 
    //like this:

    /*
        pub fn notify<T: Summary>(item1: &T, item2: &T) {}
    */


    //Specifying Multiple Trait Bounds with the + Syntax

    //We can also specify more than one trait bound. Say we wanted notify to use display 
    //formatting on item as well as the summarize method: we specify in the notify definition 
    //that item must implement both Display and Summary. We can do so using the + syntax:

    /*
        pub fn notify(item: &(impl Summary + Display)) {}

        or

        pub fn notify<T: Summary + Display>(item: &T) {}
    */

    //We can also use the `where` syntax to define which type and generics will be a specific
    //type. Instead of use declare this way:

    use std::fmt::{Display, Debug};

    pub fn some_function<T: Display + Summary ,U: Summary + Debug>(item1: &T, item2: &U) -> i32 {
        //It's just an example, so will not write no code here, only return a number
        10
    }

    //We can write the same thing above in a concise form, being better to read:

    pub fn some_function2<T, U>(item1: &T, item2: &U) -> i32
        where T: Display + Summary,
              U: Summary + Debug
    {
        10
    }

    //This function’s signature is less cluttered: the function name, parameter list, and return 
    //type are close together, similar to a function without lots of trait bounds.

    pub fn traits3() {

        let article = NewsArticle {
            headline: "New programming languages ​​will be used in the not-too-distant future".into(),
            location: "São Paulo - Brazil".to_owned(),
            author: "Victor Silva".into(),
            content: "lalalal......".to_string()
        };

        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false
        };  

        notify(&article);
        notify2(&tweet);

    }
}