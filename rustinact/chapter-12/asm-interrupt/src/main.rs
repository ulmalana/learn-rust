use std::arch::asm;

fn main() {
    unsafe {
        asm!("int 42");
    }
}
