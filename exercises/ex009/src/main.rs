fn main() {
    // How movements, referencing, borrowing and ownership works

    // stack variables

    let mut x = 5;
    let mut y = x; // entire new copy, new data

    x = 10;
    y += 20;

    println!("x is {x} and y is {y}"); // independent

    let _msg = "hello"; // string literal immutable stack

    println!("{_msg}");

    makes_copy(y); // copy y without taking ownership or deleting y in this scope, because it is a stack variable

    println!("{y}");

    // heap variables, can grow and shrink, can change in size and in value

    let msg = String::from("hello"); // String type, can grow is in the heap

    let mut msg1 = msg; // deletes msg, it was moved to msg1

    msg1.push_str(", World!");

    println!("msg is gone msg1 is {msg1}");

    let mut msg = String::from("123");
    let mut msg1 = msg.clone(); // creates a new variable without moving and deleting the last one, repeats the same data on the heap

    msg1.push_str("456");

    println!("msg is {msg} and msg1 is {msg1}");

    takes_ownership(msg1); // takes ownership into the funtion and msg1 is moved and gone, deleted

    msg = take_and_return_ownership(msg); // takes and returns ownership of the data, it gives the data back
                                          // this way the variable is moved, and moved back, without losing the data but changing its owner, and the again changing it back

    println!("{msg}");

    let len = borrows(&msg); // don't pass the variable, only a reference, "pointer" to it

    println!("{msg} is {len} characters long");

    borrows_and_change(&mut msg); // passes a mutable reference that allows the variable to change, but won't move or lose its value

    println!("{msg}");

    let _r1 = &mut msg;
    let r2 = &mut msg; // just like with movements the first mutable reference is dropped, and invalid, there can only be one mutable reference at a time
                       // this only happens when there are mutable references, only immutable references doesn't cause this problem
                       // however you also can't have a immutable reference and a mutable one.
                       // so you can either have a mutable reference or any number of immutable references
                       // And references never point to nothing, you cant have a reference for a variable that has gone out of scope.


    println!("r1 is gone, and r2 is {r2}");

    // string slices, part of a string
    let slice = &msg[3..]; // slice is a reference of a part of a string
    println!("{slice}");   // string literals works as slices, strings with fixed length

    let words = String::from("hello world this is a string");
    let frst_wrd = get_frst_wrd(&words);

    println!("first word from {words} is '{frst_wrd}'");

    // slices can also be of other types

    let arr = [1, 2, 4, 8, 16];
    let slice = &arr[2..3];

    println!("{} {}", slice[0], slice[1]);
}

fn takes_ownership(s: String) {
    println!("{s}");
} // s is out of scope and data from msg1 and s is deleted, freed

fn makes_copy(n: i8) {
    println!("{n}");
} // n is created and deleted in this function, when it goes out of scope, but the original y reamins intact

fn take_and_return_ownership(s: String) -> String {
    println!("{s}");
    s
} // s is created in this function and returned after to be stored in a variable

fn borrows(s: &String) -> usize {
    // only borrows the string with a reference, a pointer, no moving the variable and not dropping it
    println!("{s}");
    s.len()
}

fn borrows_and_change(s: &mut String) {
    // borrows with a mutable reference and changes the data
    s.push_str("abc");
}

fn get_frst_wrd(s: &str) -> &str { // returns slice doesn't take ownership of any variable
    let bytes = s.as_bytes();

    for (i, &letter) in bytes.iter().enumerate() {
        if letter == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
