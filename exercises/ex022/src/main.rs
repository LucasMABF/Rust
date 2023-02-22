// changed cargo.toml file to change behaviour on optimization when running cargo build and cargo build --realease, on the profiles settings.
use ex022;

/// when there is a library and a binary the binary is ignored on cargo doc
/// that's why this won't appear in the documentation.
/// Although if this was the only file it would work with cargo doc.
fn main(){
    ex022::hello();
    let answer = ex022::add_one(5);
    println!("{answer}");
    assert_eq!(6, answer);
    let num = ex022::get_num_from_str("42");
    println!("{num:?}");
    assert_eq!(42, num.unwrap());
}
