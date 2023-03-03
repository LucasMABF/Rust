use std::slice;

static HELLO_WORLD: &str = "Hello, World!"; // global variable
static mut COUNTER: u32 = 0; // mutable static global variable

fn main() {
    // advanced features
    // unsafe rust
    let mut num = 5;

    let r1 = &num as *const i32; // raw pointers
    let r2 = &mut num as *mut i32;
    let address = 0x8a8cf0f9a4usize;
    let r = address as *const i32;
    println!("{:?}", r1);
    println!("{:?}", r2);
    println!("{:?}", r);

    unsafe {
    println!("{:?}", *r1); // dereferecing raw pointer needs to have unsafe keyword
    println!("{:?}", *r2);

    let x = dangerous(r1);
    println!("{x}");
    }

    // dangerous(); // doesn't compile

    // safe abstraction
    
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let(a, b) = split_at_mut(r, 3);
    println!("a: {:?} b: {:?}", a, b);

    // extern functions

    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 is {} according to C.", abs(-3)); // using external functions is unsafe
    }

    println!("{}", HELLO_WORLD);

    add_to_count(3);
    unsafe{
        println!("COUNTER: {COUNTER}"); // reading from a mutable static variable is unsafe
    }

    let xyz = Xyz{}; // not unsafe
}

// unsafe functions
unsafe fn dangerous(x : *const i32) -> i32{ // dereferencing raw pointer
    *x
} 

// implementation copying standard library method
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut[i32]){
    let len = values.len();
    let ptr = values.as_mut_ptr();
    
    assert!(mid <= len);

    unsafe{
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// extern functions to use from rust in other languages
#[no_mangle] // keep the name as is
pub extern "C" fn call_from_c() {
    println!("just called a Rust function from C!");
}

fn add_to_count(inc: u32) {
    unsafe{
        COUNTER += inc; // changing mutable static variables is unsafe
    }
}

unsafe trait Foo {

}
struct Xyz;
unsafe impl Foo for Xyz{}
