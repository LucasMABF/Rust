use ex027::Post;

pub mod type_impl;

fn main() {
    // Traits and trait objects
    // Object oriented features of rust
    let screen = Screen{
        components: vec![
            Box::new(SelectBox {
                width: 10,
                height: 10,
                text: String::from("This is a select box"),
            }),
            Box::new(Button{
                width: 30,
                height: 15,
                text: String::from("This is a button."),
            }),
        ],
    };

    screen.run();

    // oop implementation of state
    
    let mut post = Post::new();

    post.add_text("Hello, World!");
    println!("{}", post.content());

    post.request_review();
    println!("{}", post.content());

    post.reject();
    println!("{}", post.content());

    post.request_review();
    post.approve();
    post.add_text("Hello, World! again!");
    post.approve();
    println!("{}", post.content());
    
    // Rust's type implementation of state
    let mut post1 = type_impl::Post::new();
    post1.add_text("Hello, World 2!, the SQL");// sequel
    let post1 = post1.request_review();
    let mut post1 = post1.reject();
    post1.add_text("\ncommit: changes made");
    let post1 = post1.request_review();
    let post1 = post1.approve();
    let post1 = post1.approve();
    println!("{}", post1.content());
}

pub trait Draw {
    fn draw(&self){
        println!("Something was drawed into the screen.");
    }
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // trait Object
    // this makes sure all values in this vector implement the trait draw
    // however they can be of different types, this couldn't be done with generics and trait bounds.
    // this is a littble bit less optimized, however it adds functionality, it is a trade-off.
}

impl Screen{
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub text: String,
}

impl Draw for Button{
    fn draw(&self){
        println!("{}, was drawed in the screen", self.text);
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub text: String,
}

impl Draw for SelectBox{}