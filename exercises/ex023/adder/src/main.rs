// as projects get larger it might be usefull to create a cargo workspace
// this works with multiple crates that might depend on each other
// and you can manage them in one single directory with cargo
// keeping all the dependacies in a cargo.lock file and only a target directory.
// and being able to run and test it with a single command.

fn main() {
    let num = 10;
    println!("{num} plus one is {}!", add_one::add_one(num));
    println!("{num} plus two is {}!", add_two::add_two(num));
}
