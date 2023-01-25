use std::io;

fn main() {
    let mut lastn = 0;
    let n = loop {
        let mut n = String::new();
        println!("Input a number: ");

        io::stdin()
            .read_line(&mut n)
            .expect("process failed to read line.");
        let n: u8 = match n.trim().parse() {
            Ok(n) => n,
            Err(_) => break lastn,
        };
        lastn = n;

        if n > 100 {
            println!("{n} is greater than 100");
        } else if n > 10 {
            println!("{n} is greater than 10");
        } else {
            println!("{n} isn't greater than 10");
        }
    };
    let condition = if n > 100 { true } else { false }; // if are expressions and can be assigned to a variable

    let condition2 = if condition { 5 } else { 2 };

    println!("{condition2}");

    let mut count = 0;
    'counting_up: loop {
        // loop identifier
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }
        count += 1;
    }
    println!("final count = {count}");

    let mut number = 6;
    while number != 0 {
        number -= 1;
        println!("{number}");
    }
    println!("LIFTOFF!!");

    let arr: [u8; 5] = [1, 3, 100, 73, 42];
    for element in arr {
        println!("{element}");
    }

    for n in (0..6).rev() {
        println!("{n}");
    }
    println!("END OF COUNTDOWM");
}
