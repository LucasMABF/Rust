#[derive(Debug)]
enum Types {
    Int(i32),
    B(bool),
    Text(String),
}

fn main() {
    // collections
    // creates empy vector
    let v: Vec<i32> = Vec::new(); // creates a new vector, (list) unknown size can grow and shrink, same data type all its elements.

    println!("{:?}", v); // borrow authomatically

    // creates vector with values with macro
    let v = vec![1, 2, 3]; // infered i32 type

    println!("{:?}", v);

    let mut v = Vec::new(); // no type annotatin, Rust can infer from following lines
                            // adding values
    v.push(5);
    v.push(6);

    println!("{:?}", v);
    // getting values inside Vec
    let second: &i32 = &v[1]; // could be v[1] instead, because it has the Copy trait.
                              // v.push(7); can't do because of immutable borrow in line above, since this is a mutable borrow.
    println!("the second element of v is {}", second);
    let second = v.get(1);
    match second {
        Some(s) => println!("the second element of v is {}", s),
        None => println!("There is no second element."),
    }
    println!("{:?}", v);

    let mut v = Vec::new();

    v.push(String::from("hello"));
    v.push(String::from("Bye"));

    let second = &v[0]; // cannot be v[0], because it can't copy without moving and can't move outside of Vec

    println!("the second element of v is {}", second);

    v.push(String::from("third"));
    let second = v.get(1);
    match second {
        Some(s) => println!("the second element of v is {}", s),
        None => println!("There is no second element."),
    }
    println!("{:?}", v);

    // iterating
    for i in &v {
        // if not &v, values are muved and v can't be used anymore
        println!("{i}");
    }

    let mut v = vec![1, 2, 3, 4];
    for i in &v { // iven if int cant not take the & off, because Vec doesn't have Copy trait
        println!("{i}");
    }

    println!("{:?}", v);
    for i in &mut v{ 
        *i += 50;
    }
    println!("{:?}", v);

    // different type items in vector through enums

    let v = vec![
        Types::Int(10),
        Types::B(true),
        Types::Text(String::from("hey there!!")),
    ];
    println!("{:?}", v);
}
