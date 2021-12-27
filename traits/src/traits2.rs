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


}