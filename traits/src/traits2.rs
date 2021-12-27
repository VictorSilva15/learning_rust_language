pub mod traits2 {
    //Continue

    //Returning Types that Implement Traits

    pub trait Informations{
        fn summarize(&self) -> String{
            format!("Read more...")
        }
    }

    pub struct Car {
        mark: String,
        model: String,
        color: (u32, u32, u32),
        age: u32,
        new: bool
    }

    impl Informations for Car {
        fn summarize(&self) -> String {
            format!("\n---Info Car---\n\nCar model: {} - {}\nColor: {:?}\nAge: {}\nNew: {}", 
            self.model, self.mark, self.color, self.age, self.new
            )
        }
    }

    pub struct Motorcycle {
        mark: String,
        model: String,
        displacements: u32,
        age: u32,
        new: bool,
        sporty: bool,
    }

    impl Informations for Motorcycle {
        fn summarize(&self) -> String {
            format!("\n---Info Motorcycle---\n\nMotorcycle Model: {} - {}\nDisplacements: {}\nAge: {}\nNew: {}\nSporty: {}",
            self.mark, self.model, self.displacements, self.age, self.new, self.sporty
            )
        }
    }

    //pub fn show(item1: &impl Informations) {
    //    println!("{}", item1.summarize());
    //}

    /*
    or:

    pub fn show<T: Informations>(item1: &T) {
        ....
    }
    */


    // Showing two parameters instead of as above, which shows one item
    pub fn show(item1: &impl Informations, item2: &impl Informations) {
        println!("{}\n{}",item1.summarize(), item2.summarize());
    }



    //-----------------------------------------------------------------------------



    //We can also use the impl Trait syntax in the return position to return a value of some 
    //type that implements a trait, as shown here:

    fn return_informations() -> impl Informations {
        Car {
            mark: String::from("Fiat"),
            model: String::from("UNO"),
            color: (255, 0, 0),
            age: 2013,
            new: false,
        }
    }


    pub fn traits1() {
        let my_car = Car {
            mark: String::from("Chevrolet"),
            model: String::from("Onix"),
            color: (255, 0, 0),
            age: 2021,
            new: true
        };

        let my_motorcycle = Motorcycle {
            mark: String::from("Kawasaki"),
            model: String::from("ZX-10R"),
            displacements: 1000,
            age: 2018,
            new: false,
            sporty: true,
        };

        //show(&my_car);
        //show(&my_motorcycle);

        show(&my_car, &my_motorcycle);

        let another_car = return_informations();
        println!("{}", another_car.summarize());
    }


    /* 
    
    Using Trait Bounds to Conditionally Implement Methods

    By using a trait bound with an impl block that uses generic type parameters, 
    we can implement methods conditionally for types that implement the specified traits. 
    For example, the type Pair<T> in Listing 10-16 always implements the new function. 
    But Pair<T> only implements the cmp_display method if its inner type T implements the 
    PartialOrd trait that enables comparison and the Display trait that enables printing.


    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    We can also conditionally implement a trait for any type that implements another trait. 
    Implementations of a trait on any type that satisfies the trait bounds are called blanket 
    implementations and are extensively used in the Rust standard library. For example, the 
    standard library implements the ToString trait on any type that implements the Display 
    trait. The impl block in the standard library looks similar to this code:

    impl<T: Display> ToString for T {
        // --snip--
    }

    Because the standard library has this blanket implementation, we can call the to_string method
    defined by the ToString trait on any type that implements the Display trait. For example, 
    we can turn integers into their corresponding String values like this because integers 
    implement Display:

    let s = 3.to_string();

    Blanket implementations appear in the documentation for the trait in the “Implementors” 
    section.

    Traits and trait bounds let us write code that uses generic type parameters to reduce 
    duplication but also specify to the compiler that we want the generic type to have particular
    behavior. The compiler can then use the trait bound information to check that all the 
    concrete types used with our code provide the correct behavior. In dynamically typed 
    languages, we would get an error at runtime if we called a method on a type which didn’t 
    define the method. But Rust moves these errors to compile time so we’re forced to fix the 
    problems before our code is even able to run. Additionally, we don’t have to write code that 
    checks for behavior at runtime because we’ve already checked at compile time. Doing so 
    improves performance without having to give up the flexibility of generics.

    Another kind of generic that we’ve already been using is called lifetimes. Rather than 
    ensuring that a type has the behavior we want, lifetimes ensure that references are valid as 
    long as we need them to be. Let’s look at how lifetimes do that.
    
    */


}