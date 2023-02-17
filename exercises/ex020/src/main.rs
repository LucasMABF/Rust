use std::collections::HashMap;

fn main() {
    // closures
    let store = Inventory{
        shirts: HashMap::from([
            (ShirtColor::Red, 10),
            (ShirtColor::Blue, 20),
            (ShirtColor::Green, 14),
            (ShirtColor::Yellow, 5)])
    };

    let user1_shirt = store.giveaway(Some(ShirtColor::Yellow));
    let user2_shirt = store.giveaway(None);
    println!("user 1 is getting a {:?} shirt and user 2 is getting a {:?} shirt", user1_shirt, user2_shirt);

    // annotating on closures
    let add_one = |x:u32| -> u32{x+1};   
    println!("{}", add_one(0));

    // ummutably borrowing clousures - Fn

    let mut list = vec!(1, 2, 3);
    let borrows = || println!("{:?}", list); // closure borrows a reference to list, immutably
    borrows(); // calls the closure
    println!("{:?}", list);

    // mutably borrowing closures - FnMut

    let mut mut_borrows = || list.push(4);
    // println!("{:?}", list); // cannot have a immutable borrow when a mutable is being used
    mut_borrows();
    println!("{:?}", list);

    // closures that take ownership - FnOnce

    let takes_ownership = move || {
        list.push(5);
        list
    };
    let list = give_ownership_back(takes_ownership);
    println!("{:?}", list); // value moved to closure

    
    let mut rect_list = [Rectangle{width: 10, height: 10}, Rectangle{width: 5, height: 30}, Rectangle{width: 5, height: 5}];
    let mut count = 0;

    rect_list.sort_by_key(|r|{
        count += 1;
        r.width * r.height
    });

    println!("{:?}", rect_list);
    println!("{count} operations");

}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn give_ownership_back<F>(f: F) -> Vec<i32>
where
    F: FnOnce() -> Vec<i32> // Closure type bound
{
    let mut list = f();
    println!("{:?}", list);
    list.push(6);
    list
}

#[derive(Debug,  PartialEq, Copy, Clone, Eq, Hash)]
enum ShirtColor{
    Red,
    Blue,
    Green,
    Yellow,
}

struct Inventory{
    shirts: HashMap<ShirtColor, u32>,
}

impl Inventory{
    fn giveaway(&self, pref: Option<ShirtColor>) -> ShirtColor{
        pref.unwrap_or_else(|| self.most_stocked_shirt()) // closure used here to return ShirtColor that has more in stock
    }

    fn most_stocked_shirt(&self) -> ShirtColor{
        let mut most_stocked: ShirtColor = ShirtColor::Red;
        let mut most = 0;
        for (color, inventory) in &self.shirts{
            if *inventory > most{
                most = *inventory;
                most_stocked = *color;
            }
        }
        most_stocked
    }
}
