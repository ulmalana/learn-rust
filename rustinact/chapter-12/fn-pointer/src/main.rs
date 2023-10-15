fn noop() {}

fn main() {
    let fn_ptr = noop as usize;
    let typed_fn_ptr = noop as *const fn() -> ();

    println!("noop as usize: 0x{:x}", fn_ptr);
    println!("noop as *const T: {:p}", typed_fn_ptr);
}
