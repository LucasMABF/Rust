fn main() {
    // iterators
    
    let mut v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // uses immutable references to each item
    println!("{:?}", v1); // doesn't consume v1
    
    for val in v1_iter{ // consume iterator
        println!("{val}");
    }

    // println!{"{:?}", v1_iter}; // can't do this, value moved
    let mut v1_iter = v1.iter(); // needs to be mutable to call next, since it consumes its items

    for _i in 0..4{ // 0..4 return an iterater withing that range
        let x = v1_iter.next(); // returns Option consumes each item
        println!("{:?}", x);
    }
    
    let v1_iter = v1.iter(); 
    let sum: i32 = v1_iter.sum(); // consumes v1_iter, needs type annotation
    println!("{sum}");

    let v1_iter = v1.iter(); 
    let new_iter = v1_iter.map(|x| x * 10); // consumes iter and gives back a new one
    let v2: Vec<_> = new_iter.collect(); // collects the iterator to a collection data type, to make sure it was run
    for i in v2{ // consumes v2, because it implicitly calls into_iter, that consumes the vec
        println!("{i}");
    }
    // println!("{:?}", v2);

    let v1_iter = v1.iter();
    let filt = vec![2, 3, 4];
    let filt_iter = v1_iter.filter(|x| filt.contains(x));
    for i in filt_iter{
        println!("{i}");
    }
    println!("{:?}", filt); // closure only borrows filt

    let v1_iter = v1.iter_mut(); // iterate over mutable references
    for i in v1_iter{
        *i += 100;
        println!("{i}");
    }

    println!("{:?}", v1); // doesn't move v1, only changes it;

    let v1_iter = v1.into_iter(); // into iter takes ownership of the v1, and has owned values 
    for mut i in v1_iter{
        i += 200;
        println!("{i}");
    }
    // println!("{:?}", v1); // moved

}
