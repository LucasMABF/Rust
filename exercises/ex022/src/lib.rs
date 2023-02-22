//! # Whole crate
//! this is an exercise to test and showcase examples of documenting in rust
//! this comments document the item that they contain them, not the following item
//! In this case they comment the root of the library

/// Documentation comments are written this way, with three slashes
/// they convert into HTML, and support MarkDown syntax
/// they are supposed to come before the code they are doccumenting
/// This function simply prints ***Hello, world!*** to the terminal
/// through the `println!` macro.
/// you can create a examples section this way:
pub fn hello() {
    println!("Hello, world!");
}


/// this function adds 1 to a number given
/// it gets a. i32 and returns an i32.
/// # examples
/// ```
/// use ex022::add_one;
/// let n = 5;
/// let answer = add_one(n);
/// assert_eq!(6, answer);
/// ```
pub fn add_one(n: i32) -> i32{
    n + 1
}


/// Other commonly used headers are:
/// # Examples
/// Shows examples on how to use the function
/// ```
/// use ex022::get_num_from_str;
/// get_num_from_str("73");
/// ```
/// code in the documentation as above will run when you run cargo test
/// # Panics
/// Explains situations in which the code will panic
/// like using a invalid string to get a number
/// ```should_panic
/// use ex022::get_num_from_str;
/// get_num_from_str("asdf");
/// ```
/// panics with message "invalid string";
/// # Errors
/// Explains common errors that can occur
/// and cause the program to return a Result Err
/// ```
/// use ex022::get_num_from_str;
/// get_num_from_str("100");
/// ```
/// returns error with message "Not good numbers"
/// # Safety
/// if the function is unsafe it explains why and how to use it
/// # you can also create your own!
/// new section, created by me!
pub fn get_num_from_str(num: &str) -> Result<i32, String>{
    let num: i32 = num.parse().expect("Invalid String");
    if num != 42 && num != 73{
        Err("Not good numbers".to_string())
    }else{
        Ok(num)
    }   
}

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;


/// module for defining colors.
pub mod kinds{

    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor{
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor{
        Orange,
        Green,
        Purple,
    }
}

pub mod utils{
    //! utils module for working with collors

    use crate::kinds::*;

    /// combines two primary colors in equal amounts to create a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor{
        SecondaryColor::Green
    }
}
