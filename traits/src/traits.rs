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