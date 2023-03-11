use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};
use std::fs;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self, Sender, Receiver};
use ex034::ThreadPool;

fn main() {
    // Final project
    let listnr = TcpListener::bind("127.0.0.1:7878").unwrap(); // 7878 is rust on numpad
    let pool = ThreadPool::new(10);
    let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
    for stream in listnr.incoming(){
        let stream = stream.unwrap();
        let tx = tx.clone();
        pool.spawn(move || {
            handle_connection(stream, tx);
            println!("Connection stablished!");
        });
        if let Ok(x) = rx.recv(){
            if x{
                break;
            }
        }
    }
    println!("Shutting down!");
}

fn handle_connection(mut stream: TcpStream, tx: Sender<bool>){
    let buf_reader = BufReader::new(&mut stream);
    let request = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = match &request[..] {
        "GET / HTTP/1.1" => {
            tx.send(false).unwrap();
            ("HTTP/1.1 200 OK", "hello.html")
        },
        "GET /sleep HTTP/1.1" => {
            tx.send(false).unwrap();
            println!("sleeping");
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        "GET /shutdown HTTP/1.1" => {
            println!("shutdown command received.");
            tx.send(true).unwrap();
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ => { 
            tx.send(false).unwrap();
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        }
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
