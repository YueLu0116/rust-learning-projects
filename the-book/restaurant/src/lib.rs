mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(){
            println!("We will got to juqi tonight!");
        }
        //fn seat_at_table() {}
    }

    // mod serving {
    //     fn take_order() {}

    //     fn serve_order() {}

    //     fn take_payment() {}
    // }
}

mod back_of_house{
    // pub struct only makes itself public, excluding all the members.
    pub struct Breakfast{
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast{
        pub fn summer(toast: &str)->Breakfast{
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    // pub enum makes all its members pub
    pub enum Appetizer{
        Bread,
        Salad,
    }
}

// pub use makes the using available to external codes.
pub use crate::front_of_house::hosting;

// in the same level as front_of_house's
// so, mod front_of_house needn't be pub
pub fn eat_at_restaurant(){
    // use absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    
    // use relative path
    // eat_at_restaturant is at the same level of front_of_house
    // front_of_house::hosting::add_to_waitlist();

    // use keyword, note "hosting" is a must
    hosting::add_to_waitlist();

    let _apt = back_of_house::Appetizer::Salad;
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I want to change to {} toast. Thanks!", meal.toast);
}