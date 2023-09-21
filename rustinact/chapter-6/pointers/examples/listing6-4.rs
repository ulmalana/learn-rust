fn main() {
    let a: i64 = 42;
    let a_ref = &a;
    let a_ptr = &a as *const i64;
    println!("a: {} (ref: {:p}) (pointer: {:p})", a, a_ref, a_ptr);
    // assert_eq!(a_ref, a_ptr);
}
