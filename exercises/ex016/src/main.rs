use std::fs::File;
use std::io;
use std::io::Read;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{ // returns any kind of error or OK(()), exits with 0 for Ok and any other number for error
    // Error handling
    // panic!("So long and thanks for all the fish!"); // panics and abors unwinding the data
    // panic!("Or better yet DONT PANIC!");

    let v = vec![1, 2, 3, 4];

    // println!("{}", v[99]); // panics index out of range

    // Results enum and match

    let f_res = File::open("hello.txt");

    let f = match f_res {
        Ok(file) => file,
        Err(error) => match error.kind(){
            io::ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        }
    };

    // unwrap
    let f = File::open("hello.txt").unwrap(); // takes the value inside of Ok or returns an error

    let f = File::open("hello.txt").expect("File hello.txt not found and should be included"); // unwrap with given panic message

    let name = read_username_file(&f).expect("Error in read_username_file function");

    println!("{name}");

    let f = File::open("hello.txt").expect("File hello.txt not found and should be included"); // unwrap with given panic message

    let name = read_username_file2(&f).expect("Error in read_username_file function");

    println!("{name}");

    let n = None;

    println!("{:?}", times_two(n).unwrap_or(100)); // can also use unwrap_or_else to run a block of code if None or Err

    println!("{:?}", times_two(Some(8)).unwrap());

    let f = File::open("hello.txt").expect("File hello.txt not found and should be included"); // unwrap with given panic message

    let name = read_username_file2(&f)?; // if none returns early

    println!("{name}");

    let n = Number::new(100);
    println!("{}", n.value());
    Ok(())
}

// return an error, propagating errors
fn read_username_file(mut f: &File) -> Result<String, io::Error> {
    let mut username = String::new();
    match f.read_to_string(&mut username){
        Ok(_) => Ok(username),
        Err(e) => Err(e), // returns error
    }
}

// the ? operator
fn read_username_file2(mut f: &File) -> Result<String, io::Error>{
    let mut username = String::new();
    f.read_to_string(&mut username)?; // or use sdt::fs::read_to_string(File)
    Ok(username)
}


fn times_two(n: Option<i32>) -> Option<i32>{
    Some(n? * 2) // also can be used on Option to return early none if none or return unwraped if Some
}

// instead of error handling you can also use types to prevent unwanted outcome
pub struct Number{
    value: i32,
}

impl Number{
    pub fn new(value: i32) -> Number{
        if value < 2 || value > 100{
            panic!("The type number need to be between 1 and 100, got {}", value);
        }

        Number {
            value
        }
    }
    pub fn value(&self) -> i32{
        self.value
    }
}
