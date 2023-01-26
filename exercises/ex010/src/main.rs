fn main() {
    // Structs
    // groups pieces of data together, like a tuple, but with different names
    let mut user1 = User{
        email: String::from("someone@example.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 1,
    };

    user1.sign_in_count += 1;
    println!("name: {}, email: {}, sign count: {}, active: {}.", user1.username, user1.email, user1.sign_in_count, user1.active);

    let user2 = build_user(String::from("email@gmail.com"), String::from("someoneelse"));
    println!("name: {}, email: {}, sign count: {}, active: {}.", user2.username, user2.email, user2.sign_in_count, user2.active);

    let user3 = User{
        email: String::from("anothernewuser@newuser.com"),
        username: String::from("newUser"),
        ..user1 // this copies the rest of the fields from user one
                // if the values copied are from the stack both users are still valid and can be used
                // However if there are heap variables copied, like Strings, they were moved from one instance to another
                // that way making the user copied invalid after the move.
    };
    println!("name: {}, email: {}, sign count: {}, active: {}.", user2.username, user2.email, user2.sign_in_count, user2.active);
    // works because all values copied were from the stack
    println!("name: {}, email: {}, sign count: {}, active: {}.", user3.username, user3.email, user3.sign_in_count, user3.active);
    
    let black = Color(0, 0, 0); // color and point are different types, even though they have the same parameters
    let origin = Point(0, 0, 0);

    println!("{} {} {}, {} {} {}", black.0, black.1, black.2, origin.0, origin.1, origin.2);

    let _equal:AlwaysEqual = AlwaysEqual;

    let width1 = 30;
    let height1 = 50;

    println!("the area of the rectangle that is {} pixels wide and {} pixels high is {} square pixels.", width1, height1, width1 * height1);

    let rect1 = (20, 50); // x, y
    println!("the area of the rectangle that is {} pixels wide and {} pixels high is {} square pixels.", rect1.0, rect1.1, rect1.0 * rect1.1);

    let rect2 = Rect{
        width: 40,
        height: 60,
    };
    println!("the area of the rectangle that is {} pixels wide and {} pixels high is {} square pixels.", rect2.width , rect2.height, area(&rect2));

    println!("{:?}", rect2);

    println!("{:#?}", rect2); // pretty print

    let scale = 2;
    let rect3 = Rect{
        width: dbg!(30 * scale), // prints for debuging, needs to have Debug trait
        height: 50,
    };

    dbg!(&rect3);

    println!("the area of the rectangle that is {} pixels wide and {} pixels high is {} square pixels.", rect3.width() , rect3.height, rect3.area());

    Rect::hello();

    let rect4 = Rect{
        width: 55,
        height: 44,
    };

    let rect5 = Rect{
        width: 61,
        height: 2,
    };

    println!("can rect3 hold rect4: {}", rect3.can_hold(&rect4));
    println!("can rect3 hold rect5: {}", rect3.can_hold(&rect5));

    let sqr1 = Rect::create_square(5);
    println!("The area of a square with side {} is {}", sqr1.width, sqr1.area());
}

struct User { // struct's name
    active: bool,// struct's fields
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple structs
struct Color(i32, i32, i32); // has name for struct, bu no name for fields
struct Point(i32, i32, i32);

// unit-like structs
struct AlwaysEqual; // new type only with name and no fields, usefull for traits and behaviour

#[derive(Debug)]
struct Rect{
    width: u32,
    height: u32,
}

fn build_user(email: String, username: String) -> User{
    User {
        email, // shorthand instead of email: email
        username,
        active: true,
        sign_in_count: 0,
    }
}

fn area(rect: &Rect) -> u32{
    rect.width * rect.height
}

impl Rect{
    fn area(&self) -> u32{ // &self is a shorthand for self: &Self(that in this case references &Rect)
        self.width * self.height
    }

    fn hello(){
        println!("hello, from rect;")
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn can_hold(&self, other_rect: &Rect) -> bool{
        self.width > other_rect.width && self.height > other_rect.height
    }

}

impl Rect{ // can have more than one impl, for each struct it is best to keep them together in this case but I'll leave it this way for demonstration
    fn create_square(size: u32) -> Self{
        Self{
            width: size,
            height: size,
        }
    }
}
