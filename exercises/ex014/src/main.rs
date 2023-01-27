fn main() {
    // Strings
    let mut s = String::new(); // Strings are collections, just like Vecs, that's why we can call new()

    s.push_str("Hello, World!!"); // append &str don't take ownership

    s.push('🌎'); // push appens a char

    println!("{s}");

    let s1 = "Hello".to_string(); // converts &str literal to String type
    let s2 = ", World!!🌎".to_string();

    s = s1 + &s2; // uses a String and a &str, and takes ownership of the String, s1, has been moved and can't be used anymore
                  // &String is dereference to &str as &str[..]
    println!("{s}");

    let s1 = String::from("Hello"); // or we can use the method from to create a String from an &str
    let s2 = String::from(", World!!🌎");

    s = format!("{s1}{s2}"); // formats Strings to another String and doesn't take ownership

    println!("{s}");

    // "indexing"
    // indexing is not allowed on rust, but we can slice a String
    let hello = &s[0..6];
    let world = &s[6..18]; // emoji of the world takes 4 bytes, that way if I slice in the midlle of it the code panics!

    println!("{hello}{world}");

    // iterating over Strings

    for c in s.chars() {
        // to iterate through Strings in Rust is necessary to be explicit about what type to get on the iterator
        println!("{c}"); // you can use .chars() to get it character, or .bytes() to get each byte
    }
}
