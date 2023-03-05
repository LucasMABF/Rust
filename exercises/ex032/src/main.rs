fn main() {
    // advanced funtions and closures
    let answer = do_twice(add_one, 5);
    println!("{answer}");
    let answer = do_twice(times_two, 5);
    println!("{answer}");

    let list: Vec<Status> = (0u32..20).map(Status::Value).collect(); // passes the enum initializer function as parameter
    println!("{:?}", list);

    let answer = returns_function_pointer()(5);
    println!("{answer}");
    let answer = returs_closure()(4);
    println!("{answer}");
    let answer = do_three_times(|x| x * 3, 10);
    println!("{answer}");
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn times_two(x: i32) -> i32 {
    x * 2
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    // fn is of the function pointer type
    f(f(arg)) // functions are types, and implement all of the closure traits, Fn, FnMut, FnOnce
              // so they can be used instead of closures
}

#[derive(Debug)]
enum Status {
    Value(u32), // each enum varient's name becomes an initializer function, this case it is a function that takes an i32 and gives an Status::Values(i32)
    Stop,
}

fn returns_function_pointer() -> fn(i32) -> i32{
    times_two
}

fn returs_closure() -> impl Fn(i32) -> i32{ // Closures are traits
    if 10 > 1{
        |x| x + 1
    }else{
        |x| x * 100
    }
    
}

fn do_three_times(f: impl Fn(i32) -> i32, x: i32) -> i32{
    f(f(f(x)))
}
