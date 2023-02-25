use std::thread;
use std::sync::Mutex;
use std::sync::Arc;
use std::time::Duration;

fn main() {
    // shared state concurrency

    let m = Mutex::new(5); // smart pointer, internally changable, like RefCell for concurrency


    {
        let mut num = m.lock().unwrap(); // gets lock and returns a smart pointer wrapped
        *num = 6;
    } // returns lock

    println!("m: {:?}", m);

    let counter = Arc::new(Mutex::new(0)); // smart pointer with counted multiple owners, like Rc for concurrency
    let mut handles = vec![];

    for _ in 0..10{
        let counter = Arc::clone(&counter); // test
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();  // gets lock or wait, puts program on hold, until it gets the lock
                                                    // if the thread with the lock panics it will result in an Error returned from lock, so it needs to be unwraped
                                                    // in this case the Mutex is considered poisoned.
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap(); // wait for each thread
    }

    println!("Result: {:?}", *counter.lock().unwrap());

    // deadlocks
    // similar to reference cycles Mutex can create a deadlock
    // in wich each thread is waiting for one another 

    let x = Arc::new(Mutex::new(5));

    let y = Arc::new(Mutex::new(6));

    let a = Arc::clone(&x);
    let b = Arc::clone(&y);
    let handle = thread::spawn(move || {
        let mut num = a.lock().unwrap();
        *num += 1;
        println!("first change a done");
        thread::sleep(Duration::from_secs(2));
        let mut num2 = b.lock().unwrap(); // at this time the other thread will have b's lock, and won't finish because it is waiting for a's lock that this thread has.
        *num2 += 1;
        println!("second change b done");
    });

    let a = Arc::clone(&x);
    let b = Arc::clone(&y);
    let handle1 = thread::spawn(move || {
        let mut num = b.lock().unwrap();
        *num += 1;
        println!("first change b done");
        let mut num2 = a.lock().unwrap();// both threads will wait for each other forever
        *num2 += 1;
        println!("second change a done");
    });

    thread::sleep(Duration::from_secs(3));
    println!("{:?} {:?}", x, y);

    handle.join().unwrap();
    handle1.join().unwrap();

    // there are the traits Sync and Send to handle types for concurrency
    // but they are not safe to implement on custom types
}
