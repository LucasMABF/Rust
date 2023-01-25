fn main() {
    let mut x = 5;
    println!("the value of x is: {x}");
    x = 6;
    println!("the value of x is: {x}");

    let y = 10;
    println!("the value of y is: {y}");
    let y = y + 1;
    println!("the value of y is: {y}");
    {
        let y = y * 2;
        println!("the value of y is: {y}");
    }
    let y = y * 3;
    println!("the value of y is: {y}");

    let z: u8 = 100; // 0 to 255
    let a: i8 = -100; // - 128 to 127
    println!("z = {z} a = {a}");
    let z: i8 = z.try_into().expect("Not a valid number");
    println!("z + a = {}", z + a);

    let b: f64 = 2.1;
    let c: f32 = 3.1;
    println!("b is {b} and c is {c}");
    let c: f64 = c.try_into().expect("Not a valid float number");
    println!("c - b = {}", c - b);// imprecision

    let diff= 95.5 - 4.3;
    println!("{diff}");

    let mult = 2 * 3 * 7;
    println!("{mult} = 2 * 3 * 7");

    let div = 5 / 3;
    println!("5 // 3 = {div}");

    let div = 5.0 / 3.0;
    println!("5 / 3 = {div:.2}");

    let remainder = 5 % 3;
    println!("5 % 3 = {remainder}");

    let t = true;
    let f: bool = false;
    println!("t is {t} and f is {f}");

    let d = 'D';
    let e: char = 'E';
    let g = "😁";
    println!("d is {d}, e is {e} and g is {g}.");

    let tup: (u8, f64, char) = (200, 30.2, '😂');

    let (h, i, j) = tup;
    println!("the tuple h is ({h}, {i}, {j}).");

    println!("the second element of the tuple is {}", tup.1);

    let arr:[i8; 5] = [1, 2, 3, 4, 5];
    println!("{}", arr[1]);
    let k = [3; 4];
    println!("[{}, {}, {}, {}]", k[0], k[1], k[2], k[3]);
}
