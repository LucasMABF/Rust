use std::io;

fn main() {
    println!("Fahrenheit and Celsius converter!!");
    loop {
        let mut conversion = String::new();
        println!("Enter 1 to convert from fahrenheit to celsius;");
        println!("Enter 2 to convert from celsius to fahrenheit;");
        println!("Enter other number to quit.");
        io::stdin()
            .read_line(&mut conversion)
            .expect("Failed to read input!");

        let conversion: i32 = match conversion.trim().parse() {
            Ok(n) => n,
            Err(_) => break,
        };

        if conversion == 1 {
            let f: i32 = loop {
                let mut f = String::new();
                println!("Fahrenheit: ");
                io::stdin().read_line(&mut f).expect("Failed to read input");
                match f.trim().parse() {
                    Ok(n) => break n,
                    Err(_) => {
                        println!("please enter a valid number");
                        continue;
                    }
                };
            };
            let c = (f - 32) * 5 / 9;
            println!("{f}Fº is equal to {c}Cº");
        } else if conversion == 2 {
            let c: i32 = loop {
                let mut c = String::new();
                println!("Celsius: ");
                io::stdin().read_line(&mut c).expect("Failed to read input");
                match c.trim().parse() {
                    Ok(n) => break n,
                    Err(_) => {
                        println!("please enter a valid number");
                        continue;
                    }
                };
            };
            let f = (c * 9 / 5) + 32;
            println!("{c}Cº is equal to {f}Fº");
        } else {
            break;
        }
    }
}
