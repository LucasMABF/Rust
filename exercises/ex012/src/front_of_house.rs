pub mod hosting; // makes it accessable for parent modules

pub mod serving {
    pub fn activate_serving(){
        println!("Hello, I will be your servent tonight, call me when you are ready to place ther order.")
    }

    #[derive(Debug)]
    pub enum Meals{ // to make enums public you only need to make it public and all its options will automatically be public.
        Steak,
        Chicken,
        Fish,
    }

    #[derive(Debug)]
    pub enum Sides{
        Fries,
        Rice,
        Beans,
        Vegetables,
    }

    #[derive(Debug)]
    pub enum Drinks{
        Water,
        Soda,
        Juice,
        Beer,
        Wine,
        Scoth,
    }

    pub struct Order{ // to make structs public you need to make some of its items public too
        pub main:Meals,
        pub side: Sides,
        pub drink: Drinks,
    }

    
    pub fn take_order(order: Order){
        println!("you placed the order of {:?} with {:?} and with the drink {:?}", order.main, order.side, order.drink)
    }

    fn serve_order(){}

    fn take_payment(){}
}