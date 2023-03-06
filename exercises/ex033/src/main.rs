use ex033_derive::HelloMacro;
use ex033::HelloMacro;

#[macro_export]
macro_rules! vec{
    ( $( $x: expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
                println!("item added!");
            )*
            temp_vec
        }
    };
}

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    // macros

    // declarative macros
    let x = vec!(1, 2, 3);
    println!("{:?}", x);

    // procedural macros

    // custom #[derive]
    Pancakes::hello_macro();
}
