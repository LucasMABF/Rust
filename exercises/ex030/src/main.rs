fn main() {
    // Advanced traits
    let mut x = Numbers::new(1, 2, 3);
    println!("{:?}", x);
    let mut y = Characters('a', 'b', 'c');
    println!("{:?}", y);
    x.add(5);
    println!("{:?}", x);
    y.add('a');
    println!("{:?}", y);

    // ambiguous functions

    let person = Human;
    person.fly(); // most direct implementation
    Wizard::fly(&person); // more explicit syntax for methods
    Pilot::fly(&person);
    println!("{}", person.name()); // most direct implementation
    println!("{}", <Human as Wizard>::name()); // fully qualified syntax for non methods
    println!("{}", <Human as Pilot>::name()); 

    // supertraits
    // traits that are required for other traits
    let point = Point{
        x: 10,
        y: 15,
    };
    point.outline_print();

    // newtype
    let v = Wrapper(vec![String::from("hello"), String::from(", "), String::from("World!")]);
    println!("{v}");

}

pub trait List<A = i32> { // default generic type, doesn't need to specify if is i32
    type Item; // associated type, to be defined on each implementation

    fn new(x: Self::Item, y: Self::Item, z: Self::Item) -> Self;

    fn add(&mut self, x: A);
}
#[derive(Debug)]
struct Numbers(i32, i32, i32);
impl List for Numbers{
    type Item = i32; // associated type, specific for this implementation

    fn new(x: Self::Item, y: Self::Item, z: Self::Item) -> Numbers {
        Numbers(x, y, z)
    }

    fn add(&mut self, x: i32){
        self.0 += x;
        self.1 += x;
        self.2 += x;
    }
}

#[derive(Debug)]
struct Characters(char, char, char);
impl List<char> for Characters{ // changing default

    type Item = char; // same trait with different type

    fn new(x: Self::Item, y: Self::Item, z: Self::Item) -> Characters{
        Characters(x, y, z)
    }

    fn add(&mut self, x: char){
        self.0  = (self.0 as u8 + x as u8) as char;
        self.1  = (self.1 as u8 + x as u8) as char;
        self.2  = (self.2 as u8 + x as u8) as char;
    }
}

trait Pilot{
    fn fly(&self) {
        println!("This is your captain speaking. We are ready for takeoff.");
    }

    fn name() -> String{
        String::from("Maverick")
    }
}

trait Wizard{
    fn fly(&self) {
        println!("Grab your brooms!");
    }

    fn name() -> String{
        String::from("Harry Potter")
    }
}

struct Human;

impl Human {
    fn fly(&self) {
        println!("*waves arms furiously and fails*"); // gets preference, as it is more specific, is directly implemented on Human
    }

    fn name(&self) -> String{
        String::from("Joe")
    }
}

impl Pilot for Human {}

impl Wizard for Human {}

trait OutlinePrint: std::fmt::Display{ // requires Display supertrait for this trait to work
    fn outline_print(&self){
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point{
    x: i32,
    y: i32,
}

impl std::fmt::Display for Point { // needs to implement to use the OutlinePrint trait
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point{}


// newtype for implementing external traits on external types

struct Wrapper(Vec<String>);

impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "[{}]", self.0.join(", "))
    }
}
