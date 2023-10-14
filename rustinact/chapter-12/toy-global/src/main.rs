use rand;

static mut SHUTDOWN: bool = false;

fn main() {
    loop {
        unsafe {
            SHUTDOWN = rand::random();
        }
        print!(".");

        if unsafe { SHUTDOWN } {
            break;
        };
    }
    println!();
}
