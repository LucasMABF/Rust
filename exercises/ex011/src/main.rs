#[derive(Debug)]
enum IpAddr{
    V4(u8, u8, u8, u8), // enums can hold data
    V6(String), // and can have different types
}

#[derive(Debug)]
enum Message{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message{
    fn print_msg(&self){
        match self{
            Message::Write(msg) => println!("{msg}"),
            _  => println!("{:?}", self) // _ is a variable that collects the data from the match it is comparring
        }                                // it is possible to name this variable and to use it in the code block after the =>
                                         // this serves as a placeholder for every other option. it works as a catch-all.
    }
}
#[derive(Debug)]
enum USState{
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
}         

#[derive(Debug)]
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

// match control flow
impl Coin{
    fn value(&self) -> u8{
        match self{
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("this coin is a quarter and is from {:?}", state);
                25
            }
        }
    }

    fn name(&self) -> &str{
        match self{
            Coin::Penny => "Penny",
            Coin::Nickel => "Nickel",
            Coin::Dime => "Dime",
            Coin::Quarter(_) => "Quarter",  
        }
    }

}

fn plus_one(x: Option<i32>) -> i32{
    match x{
        None => 1,
        Some(n) => n + 1,
    }
}

fn main() {
    // enums
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback= IpAddr::V6(String::from("::1"));

    println!("{:?} {:?}", home, loopback);

    let msg = Message::Write(String::from("Hello!!"));
    msg.print_msg();

    // Option enum

    let some_number = Some(5);
    let some_char = Some('e');

    println!("{:?}", some_number);
    let some_number:i32 = plus_one(some_number);
    println!("{some_number}");

    // if let control
    // how to deal with enums without having to create a let

    if let Some(c) = some_char{
        println!("some char: {c}")
    }

    let null_number: Option<i32> = None;

    let x = Some("value");
    let x = x.expect("wtf"); // if it were None it would crash with message "wtf"
    println!("{x}");

    let my_dime = Coin::Dime;

    println!("my {:?} is worth {} cents!", my_dime, my_dime.value());

    let my_quarter = Coin::Quarter(USState::Illinois);

    println!("my {} is worth {} cents!", my_quarter.name(), my_quarter.value());

}
