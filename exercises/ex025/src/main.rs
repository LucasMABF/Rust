use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    // Concurrency
    let handle = thread::spawn(|| { // JoinHandle to handle thread and make sure it fineshes
        for i in 1..10 {
            println!("hi number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 11..20{
        println!("hi number {} from main thread!", i);
        thread::sleep(Duration::from_millis(2));
    }

    handle.join().unwrap(); // wait for its thread to finish

    static n: i32 = 32; // static values work on different threads.

    let handle = thread::spawn(||{
        println!("Here's a vector: {:?}", n);
    });

    println!("{n}");

    handle.join().unwrap();
    
    let v= vec![1, 2, 3, 4, 5];

    let handle = thread::spawn(move ||{
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
    // println!("{:?}", v); // v has been moved to the other thread and then dropped

    // channels

    let (tx, rx) = mpsc::channel(); // transmitter and receiver deconstructed to tx and rx

    thread::spawn(move || {
        let val = String::from("hi from the other thread!");
        tx.send(val).unwrap(); // send message
        println!("first message sent!");
        thread::sleep(Duration::from_millis(1000));
        tx.send(String::from("do some work while you wait for this message: hello!")).unwrap();
        println!("second message sent!");
    });
    let received = rx.recv().unwrap(); // receive message, waits untill message is received
    println!("{received}");


    let received2 = loop {
        println!("Work");
        thread::sleep(Duration::from_millis(200));
        match rx.try_recv(){ // doesn't wait for the message
            Err(_) => continue,
            Ok(value) => break value,
        }
    };

    println!("{received2}");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move||{
        let v = vec!["hello", "there", "I", "am", "in", "another", "thread"];
        for e in v{
            tx.send(e).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        println!("ended");
        thread::sleep(Duration::from_secs(3));
    });

    for received in rx { // creates iterator of rx, that prints each received value, and iterarion will end when channel is close
    // waits for each value received to run the loop, doesn't skip if not found.
        println!("{received}"); 
        println!("work");
    }
    println!("exited loop"); // only runs when for loop ends, when the other thread is terminated

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone(); // to create multiple transmitters

    thread::spawn(move || {
       let vals = vec!["hi", "from", "thread", "with", "multiple", "transmiters"];
       for e in vals{
            tx1.send(e).unwrap();
            thread::sleep(Duration::from_secs(3));
       }
       println!("fineshed");
       thread::sleep(Duration::from_secs(10));
    });

    thread::spawn(move || {
        let vals = vec!["more", "&str", "for", "you"];
        for e in vals{
            tx.send(e).unwrap();
            thread::sleep(Duration::from_secs(5));
        }
        println!("fineshed2");
    });

    for received in rx { // get from both transmitters, and wait for both to finish running
        println!("Got: {received}");
    }

} // when main thread ends, all other threads are shut down, wheter or not they have fineshed running
