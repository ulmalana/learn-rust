use std::slice;

unsafe fn dangerous() {}

//wont work
// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();

//     assert!(mid <= len);

//     (&mut slice[..mid], &mut slice[mid..])
// }

//works
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
        
    }
}

//calling C code
extern "C" {
    fn abs(input: i32) -> i32;
}

//static or global variables
static HELLO_WORLD: &str = "Hello, world.";
static mut COUNTER: u32 = 0;


//changing static variables
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

//unsafe trait
unsafe trait Foo {

}

unsafe impl Foo for i32 {
    // add code here
}

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let mut v = vec![1,2,3,4,5,6];
    let r = &mut v[..];

    let (a,b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1,2,3]);
    assert_eq!(b, &mut [4,5,6]);

    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        dangerous();
        println!("Absolute value of -3 according to C: {}", abs(-3));
        println!("COUNTER: {}", COUNTER);
    }
}

