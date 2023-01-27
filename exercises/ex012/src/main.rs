// use std::{cmp::Ordering, io::{self, Write}}; how to import external items from the same crate in onle line brings into scope Ordering io and Write
// instead of 
// use std::Cmp::Ordering
// use std::io
// to bring all public items inside a path use the glob(*) operator like:
// use std::colletions::*
// brings to scope every public item in collections.

fn main(){
    println!("hello, World!");
    ex012::eat_at_restaurant(); // uses at crate lib named ex012.
}
