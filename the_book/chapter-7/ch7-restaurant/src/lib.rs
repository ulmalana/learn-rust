#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

//re-exporting front_of_house module which is in a separate file
mod front_of_house;
pub use crate::front_of_house::hosting;

mod coba_front_of_house;
pub use crate::coba_front_of_house::coba_hosting;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

}

//use crate::front_of_house::hosting;
//or
//use self::front_of_house::hosting;


pub fn eat_at_restaurant() {
    //absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //relative path
    front_of_house::hosting::add_to_waitlist();

    //simplified with use keyword
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    //different scheme
    coba_hosting::coba_add_to_waitlist();
    coba_hosting::coba_add_to_waitlist();

    //order a breakfast with rye
    let mut meal = back_of_house::Breakfast::summer("Rye");

    //change mind. rye to wheat
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn serve_order() {}
