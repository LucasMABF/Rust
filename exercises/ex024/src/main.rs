use crate::List::Cons;
use crate::List::Nil;
use crate::ListRc::{Consrc, Nilrc};
use crate::ListRef::{Consref, Nilref};
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Weak;

fn main() {
    // Smart pointers
    // box
    // heap allocated pointer

    let b = Box::new(5);
    println!("{b}");
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);

    // deref

    // on regular references

    let x = 6;
    let y = &x;

    assert_eq!(x, *y);

    // on box, smart pointer
    
    let y = Box::new(x);
    println!("{x}"); // can still use x, because it implements the Copy trait

    assert_eq!(x, *y);

    // on MyBox

    let y = MyBox::new(x);
    println!("{:?}", y);

    assert_eq!(x, *y);

    let name = MyBox::new(String::from("Rust")); // variables are dropped in the reverse order they were created, this will go first, since it is the last one in this scope

    hello(&name); 

    {MyBox::new("hey124");}

    let a = MyBox::new("Early drop");
    drop(a);
    println!("hello there");

    // Rc, reference counted smart pointer

    let x = Rc::new(Consrc(5, Rc::new(Consrc(10, Rc::new(Nilrc)))));
    println!("Rc count: {}", Rc::strong_count(&x));
    let y = Consrc(3, Rc::clone(&x));
    println!("Rc count: {}", Rc::strong_count(&x));
    {
        let z = Consrc(4, Rc::clone(&x));
        println!("Rc count: {}", Rc::strong_count(&x));
        println!("{:?}", z);
    }
    println!("{:?}", y);
    println!("Rc count: {}", Rc::strong_count(&x));
    drop(y);
    println!("Rc count: {}", Rc::strong_count(&x));
    drop(x);

    // RefCell with Rc, to have mutable and multiple owners

    let value = Rc::new(RefCell::new(1));
    let x = Rc::new(Consref(Rc::new(RefCell::new(2)), RefCell::new(Rc::new(Consref(Rc::clone(&value), RefCell::new(Rc::new(Nilref)))))));
    let y = Rc::new(Consref(Rc::new(RefCell::new(3)), RefCell::new(Rc::clone(&x))));
    let z = Consref(Rc::new(RefCell::new(4)), RefCell::new(Rc::clone(&x)));

    *value.borrow_mut() += 10; // changes internally

    println!("x: {:?}", x);
    println!("y: {:?}", y);
    println!("z: {:?}", z);

    // creating a reference cycle
    if let Some(link) = x.tail(){
        *link.borrow_mut() = Rc::clone(&y); // links x to y that is linked to x
    }

    // println!("z: {:?}", z); // overflows stack, error

    drop(y); // doesn't drop the count of x
    println!("{}", Rc::strong_count(&x)); // memory leak
    drop(x);
    drop(z);

    // using Weak references to solve memory leak and reference cycle problem
    // building a tree

    let leaf = Rc::new(Node{
        value: 4,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade()); // upgrade checks if reference still exists, otherwise returns None

    let branch = Rc::new(Node{
        value: 2,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new()),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // creates a Weak reference to Rc

    println!("tree: {:?}", branch);
    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());
    println!("Strong leaf: {}", Rc::strong_count(&leaf)); 
    println!("Strong branch: {}", Rc::strong_count(&branch)); 
    println!("Weak leaf: {}", Rc::weak_count(&leaf)); 
    println!("Weak branch: {}", Rc::weak_count(&branch)); 

    drop(branch);
    println!("after drop");
    println!("Strong leaf: {}", Rc::strong_count(&leaf)); 
    println!("Weak leaf: {}", Rc::weak_count(&leaf)); 
    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());
}

#[derive(Debug)]
enum List{
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
struct MyBox<T: core::fmt::Debug>(T);

impl<T> MyBox<T> where 
T: core::fmt::Debug,
{
    fn new(x: T) -> Self{
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>
where T: core::fmt::Debug,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> where
T: core::fmt::Debug,
{
    fn drop(&mut self){
        println!("Dropping MyBox with data: {:?}", self.0);
    }
}

fn hello(name: &str){
    println!("Hello, {name}");
}

#[derive(Debug)]
enum ListRc{
    Consrc(i32, Rc<ListRc>),
    Nilrc,
}

#[derive(Debug)]
enum ListRef{
    Consref(Rc<RefCell<i32>>, RefCell<Rc<ListRef>>),
    Nilref,
}

impl ListRef{
    fn tail(&self) -> Option<&RefCell<Rc<ListRef>>>{
        match self{
            Consref(_, item) => Some(item),
            Nilref => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>, // Weak reference, doesn't own the value and doesn't create a cycle or memory leak
}
