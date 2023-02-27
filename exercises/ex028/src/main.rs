#[allow(unused_variables)] // doesn't warn on unused variables.
fn main() {
    // Patterns
    // match patterns
    // refutable patterns and an irrefutable pattern
    // irrefutable patterns are the ones that will always match, can't fail
    // refutable is the ones that might not match.

    let x = Some(32);

    let x = match x{
        Some(i) => Some(i + 1), // some(i) is a pattern, i being a variable
        None => None,
    };

    println!("{:?}", x);

    // if let patterns 
    // refutable patterns

    let fav_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "17".parse();

    if let Some(color) = fav_color { // Some(color) is a pattern, color is a variable. Is an refutable pattern, can fail to match
        println!("Using your favorite color, {color}, for the background!");
    } else if is_tuesday{
        println!("Tuesday is green day!");
    } else if let Ok(age) = age{
        if age > 30 {
            println!("Using purple as the background color");
        } else{
            println!("Using orange as the background color");
        }
    } else{
        println!("Using blue as background color");
    }

    let mut stack = vec![1, 2, 3, 4, 5];

    // while let patterns
    // refutable patterns
    
    while let Some(top) = stack.pop() { // while let also exists, which means that it will run for as long as a pattern contieues to match
        println!("{top}");              // the last stack.pop() returns a None, that doesn't match the pattern, and will break the loop,
    }

    // for loop patterns
    // irrefutable patterns

    let v = vec!['a', 'b', 'c', 'd'];

    for (index, value) in v.iter().enumerate(){ // (index, value) is a pattern that will match with v.iter().enumerate() 
    println!("{} is at index {}", value, index);// and assign the values to the index and value variables.
                                                // this is called destructuring, breaking apart values from a tuple.
    }

    let v = vec![&32];

    for &&x in v.iter(){ // can dereference references with pattern
        println!("{}", x);
    }

    // let statements patterns
    // irrefutable patterns

    let x = 7; // x is a pattern, a irrefutable pattern, msut always match, can't fail to match

    let (x, y, z) = (1, 2, 3); // (x, y, z) is a pattern, with variables x y z that will match the other side of the expression.

    let &x = &10; // can also dereference
    println!("{}", x);

    let point = (3, 5);
    print_coords(&point);

    // matching literals
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        n => println!("more than three"),
    }

    // matching named variables
    let x = Some(50);
    let y = 10;
    match x{
        Some(10) => println!("got 10"),
        Some(y) => println!("y: {y}"),
        _ => println!("default case, x = {:?}, y = {y}", x),
    }
    println!("x: {:?}, y: {y}", x);

    // multiple patterns
    let x = 10;
    match x{
        1 | 2 | 3 => println!("1, 2, or 3!"),
        42 => println!("The answer to life the universe and everything!"),
        73 => println!("The best number"),
        _ => println!("not important number"), 
    }

    // matching ranges of values 
    let x = 5;
    match x{
        1..=5 => println!("1 through 5"),
        _ => println!("something else!"),
    }
    let x = 'L';
    match x{
        'A'..='n' => println!("letter until n lowercase"),
        _ => println!("not until n"),
    }

    // destructuring to break apart values

    // structs

    let p = Point{x: 0, y: 7};
    let Point{x:a, y:b} = p; // destrucures point and puts its values in a and b
    println!("{a}, {b}");
    let Point{x, y} = p; // shorthand with same name variables
    println!("{x}, {y}");

    // enums
    let p = Types::Tuple(32, false);
    match p{
        Types::Unit => println!("unit type enum"),
        Types::Struct{x, y} => println!("struct type with x = {x} and y = {y}"),
        Types::Tuple(0, b) => println!("first value is zero and second is {b}"),
        Types::Tuple(32, b) => println!("first value is 32 and second is {b}"), 
        _ => println!("none of the above cases"),
    }

    // ignoring values
    // _

    let x = Types::Tuple(10, true);
    match x {
        Types::Struct{x: _, y: _} => println!("Value Struct"), // _ used to match with a some value, but won't use the value inside, part of the value is ignored
        Types::Tuple(_a, _b) => println!("Value Tuple"), // values matched with _a and _b, and binded, but won't be used, the _ only stops rust from generating a warning about unused variables 
        _ => println!("value not Some"), // all other cases, all the value is ignored
    }

    // ..
    let x: (i32, bool, Option<i32>) = (23, false, None);
    match x{
        (0, ..) => println!("first value is 0"), // ignroes multiple fields
        (a, .., Some(x)) => println!("second value is {x}, and first value is {a}"), // ignores all but first and last values
        (a, ..) => println!("First value is {a}"),
    }

    // match guards
    // doesn't check exhaustiveness
    let x = Some(42);
    let y = 42;
    println!("{}",
    match x{
        Some(x) if x % 2 == 1 => "this number is odd.",
        Some(x) if x == y => "this number, 42, is the answer to life the universe and everything",
        Some(x) => "this number is even",
        None => "None",
    });

    // @
    // used to bind a value to a variable while also testing it

    let x = 10;
    match x {
        num @ 5..=10 => println!("number {num} between 5 and 10"), // creates num variable while also testing if it is in range
        _ => println!("other number"),
    }

}

// function patterns
// irrefutable patterns
#[allow(unused_variables, dead_code)] // doesn't warn on unused function
fn foo(x: i32, _: i32){ // x is a pattern, just like when assigning variables with let and _ accepts any value but won't bind to it.
    todo!() // lets the code compile but panics if called
}

fn print_coords(&(x, y): &(i32, i32)) { // dereferences too
    println!("Current location: ({}, {})", x, y);
}

struct Point{
    x: i32,
    y: i32,
}

#[allow(dead_code)]
enum Types{
    Unit,
    Struct{x: i32, y: i32},
    Tuple(i32, bool)
}
// 200 spot on