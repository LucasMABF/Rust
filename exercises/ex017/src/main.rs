use ex017::{NewsArticle, Tweet, Summary};
use core::fmt::{Debug, Display};

fn main() {
    // Generic types, traits, and lifetimes
    let l1 = largest(&[1, 2, 5, 2, 12, 30, 20, 16, 43, 32, 22]);

    let l2 = largest(&['a', 'b', 'g', 'h', 't', 'a', 'f', 'k']);

    println!("{l1} {l2}");

    let p1 = Point{x: 15, y: 20};

    let p2 = Point{x:'a', y:'d'};

    println!("{:?} {:?}, X = {}", p1, p2, p2.x());

    let p = Point2{x: 32.0, y:'h', z: 100};

    println!("{:?}", p);

    let n:n<i32, String> = n::First(32);

    println!("{:?}", n);

    let p = Point{x: 4.0, y:3.0};

    println!("{}", p.distance_from_origin());

    let tweet = Tweet::tweet("hello", "hey there", false, false);

    println!("1 new tweet: {}", tweet.summarize());
    println!("{}", tweet.from());

    let news = NewsArticle{
        headline: String::from("Penguins win the Stanley cup championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Rust's Book"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    println!("{}", news.from());

    notify(&news);

    get_from(&news, &news);

    let a = String::from("1234");
    let b = "123456";

    let longest = longest(a.as_str(), b);

    println!("The longest string is {}", longest);

    let text = String::from("1st paragraph : asdflkjasdfljdfjs 2nd paragraph: hellow how are you");

    let part = text.split("2nd").next().expect("Error on splitting the text");

    let i = importantParagraph{
        paragraph: part,
    };

    println!("{:?}", i);

    println!("{}", first_word(&text));

    println!("{}", i.level("asdf a"));
    
    let s: &'static str = "Static lifetime"; // has static lifetime, always valid, can be used on all the program
    let a = "hello"; // also static, all string literals are static
    println!("{s}/{a}");
    
    println!("{}", longest_with_an_announcement(s, a, 288));

}

// function that accepts more than one type, using generics, the <> syntax
// have no cost at runtime, due to "translation" in compiling
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T{
    let mut largest = &list[0];

    for item in list{
        if item > largest{
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T: std::marker::Copy>{
    x: T,
    y: T,
}

impl<T: std::marker::Copy> Point<T>{
    fn x(&self) -> T{ // could also be a generic function inside a generic implementation. with generic new arguments
        self.x
    }
}

impl Point<f32>{
    fn distance_from_origin(&self) -> f32{
        (self.x.powi(2) + self.y.powi(2)).sqrt() // a**2 = b**2 + c**2
    }
}

#[derive(Debug)]
struct Point2<T, U>{
    x: T,
    y: U,
    z: i32,
}

#[derive(Debug)]
enum n<T, S>{
    First(T),
    Second(S),
    Third,
}


pub fn notify(item: &impl Summary){ // shorthand 
    println!("Breaking news! {}", item.summarize());
}

pub fn get_from<T:Summary>(item: &T, second_item: &T){ // longer form for trait as paramater, forces both paramaters to be the same type, which implements the trait
    println!("Text from {} and the other from {}", item.from(), second_item.from());
}

// pub fn some_function<T:Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> impl Display{} // you can use + to require more than one trait
                                                                                          // needs to return the same type
// pub fn other_function<T, U>(t: &T, u: &U) -> i32 // same as above, for separating the bounds
// where
// T:Display + Clone,
// U: Clone + Debug
// {}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len(){
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct importantParagraph<'a> {
    paragraph: &'a str,
}

// elisions

// one input and one output

// more than one input

// methods with self

fn first_word(s: &str) -> &str{ // no need to use lifetimes, because the compiler "guesses" that the return has the same lifetime as the paramater(because there is only one)
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i]
        }
    }
    &s[..]
}

impl<'a> importantParagraph<'a> {
    fn level(&self, txt: &str) -> &str{
        println!("message: {}", txt);
        self.paragraph
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T:Display{ // lifetimes with trait bounds and generic types
    println!("{ann}");
    if x.len() > y.len() {
        x
    } else{
        y
    }
}
