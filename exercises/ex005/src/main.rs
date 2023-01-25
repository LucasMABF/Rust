fn main() {
    another_funtion(5, '👍'); // arguments

    another_funtion(42, '🌌');
    
    let y:u8 = { // expression block returns 8
        let x = 3; // statement doesn't return anything only sets the variable x
        x + 5 // returns x + 5, no semicollon. expressions don't have semiconllon at the end statements do, adding a semicollon turns it into a statement!
    };
    println!("y is equal to {y}");

    let addition = add(4, 10);
    println!("the addition of 4 and 10 equals {addition}.");
}

fn another_funtion(x: u8, msg: char){ // paramenters
    println!("another function!!");
    println!("the value o the argument x passed is {x}");
    println!("The massege passsed is {msg}.");
}

fn add(a:u8, b:u8) -> u8{
    a + b
}
