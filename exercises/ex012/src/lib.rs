mod front_of_house;

pub use crate::front_of_house::hosting; // bring hosting to scope from absolut path, now we only need to use hosting::
                                        // and re-exports it, making it avaiable on the root module and anyone who accesses it.
use front_of_house::serving; // bring serving to scope from relative path
use front_of_house::serving::Meals as MealOptions; // brings the enum with hole path so we dont need to use the :: syntax
                                                  // and creates an alias to rename the enum on this scope

pub fn eat_at_restaurant(){
    // Absolut path
    crate::front_of_house::hosting::add_to_waitlist("first on wailtist");

    // Relative path
    front_of_house::hosting::add_to_waitlist("second on waitlist");

    hosting::seat_at_table(); // because of use we can do that 

    serving::take_order(serving::Order{
        main: MealOptions::Steak, // because of use as we can do that 
        side: serving::Sides::Fries,
        drink: serving::Drinks::Beer,
    })



}
