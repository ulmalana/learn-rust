use std::{process, thread::sleep, time};

fn main() {
    let delay = time::Duration::from_secs(1);
    let pid = process::id();

    println!("PID: {}", pid);

    for i in 1..=60 {
        sleep(delay);
        println!(". {}", i);
    }
}
