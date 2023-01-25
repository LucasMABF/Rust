use std::io;

fn main() {
    println!("Fibonacci Sequence");
    loop {
        println!("How many numbers of the fibbonacci sequence you want to see?[letters and numbers > 40 quit] ");
        let mut numbers = String::new();
        io::stdin()
            .read_line(&mut numbers)
            .expect("Failed to read input");

        let numbers: u32 = match numbers.trim().parse() {
            Ok(n) => n,
            Err(_) => break,
        };

        if numbers > 40 {
            break;
        }

        let mut last1 = 1;
        let mut last2 = 1;
        for n in 0..numbers {
            if n < 2 {
                println!("1");
                continue;
            } else {
                println!("{}", last1 + last2);
            }
            let old_last2 = last2;
            last2 = last1;
            last1 = last1 + old_last2;
        }
    }
}
