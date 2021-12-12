use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    //println!("Hello, world!");
    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
            ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));    
        }    
        
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
            ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));    
        }    
        
    });

    //let received = rx.recv().unwrap();
    for received in rx {
        println!("Got: {}", received);    
    }
    
}
