use std::thread;
use std::time::Duration;

fn main() {
    //println!("Hello, world!");
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread.", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // finish the spawned thread first, then run the main thread below
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread.", i);
        thread::sleep(Duration::from_millis(1));
    }

    // finish the main thread first, then wait for unfinished spawned thread.
    //handle.join().unwrap();

    let v = vec![1,2,3];

    let handle2 = thread::spawn(move || {
        println!("here is a vector: {:?}", v);
    });

    handle2.join().unwrap();
}
