use std::ops::Add;

fn main() {
    // advanced types

    // newtypes for wrapping existing types
    // prevent type confusion
    // let x = Meters(30) + Milimeters(30) // can't add
    let x = Meters(30) + Meters(30);
    let y = Meters(30) + Milimeters(2000).to_meters();
    println!("{:?}, {:?}", x, y);
    let z: Kilometers = 5 as Kilometers; // not new type, same as i32
    println!("5 km + 5 = {}", z + 5);

    // never type
    let w = Some(10);
    let v = match w {// ! can be coerced to other types
        Some(x) => x, // returns i32
        None => panic!("None value"), // returns !, but match still valid
    };
    println!("{v}");
}
#[derive(Debug)]
struct Meters(u32);
#[derive(Debug)]
struct Milimeters(u32);

impl Add for Meters {
    type Output = Meters;

    fn add(self, other: Meters) -> Self::Output {
        Meters(self.0 + other.0)
    }
}
impl Meters {
    fn to_mil(self) -> Milimeters {
        Milimeters(self.0 * 1000)
    }
}

impl Add for Milimeters {
    type Output = Milimeters;

    fn add(self, other: Milimeters) -> Self::Output {
        Milimeters(self.0 + other.0)
    }
}
impl Milimeters {
    fn to_meters(self) -> Meters {
        Meters(self.0 / 1000)
    }
}

// Synonyms type alias
// can be used to abreviate long types
type Kilometers = i32;

// never type
// diverging function, returns never
fn never() -> !{// never returns.
    panic!("hello");
}

fn generic<T: ?Sized + std::fmt::Debug>(t: &T){} // generics always have Sized trait, but this syntax allows for both, Sized and not Sized
