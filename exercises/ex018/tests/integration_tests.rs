use ex018;
mod common;
// only runs if all unit tests passed
// can't test the main.rs file
#[test]
fn hello_name(){
    let x = common::setup;
    assert!(ex018::hello("lucas").contains("lucas"));
}