#![feature(link_llvm_intrinsics)]
#![allow(non_camel_case_types)]
#![cfg(not(windows))]

use libc::{SIGALRM, SIGHUP, SIGQUIT, SIGTERM, SIGUSR1};
use std::mem;

const JMP_BUF_WIDTH: usize = mem::size_of::<usize>() * 8;
type jmp_buf = [i8; JMP_BUF_WIDTH];

static mut SHUTDOWN: bool = false;
static mut RETURN_HERE: jmp_buf = [0; JMP_BUF_WIDTH];
const MOCK_SIGNAL_AT: usize = 3;

extern "C" {
    #[link_name = "llvm.eh.sjlj.setjmp"]
    pub fn stjmp(_: *mut i8) -> i32;

    #[link_name = "llvm.eh.sjlj.longjmp"]
    pub fn longjmp(_: *mut i8);
}

#[inline]
fn ptr_to_jmp_buf() -> *mut i8 {
    unsafe { &RETURN_HERE as *const i8 as *mut i8 }
}

#[inline]
fn return_early() {
    let franken_pointer = ptr_to_jmp_buf();
    unsafe { longjmp(franken_pointer) };
}

fn register_signal_handler() {
    unsafe {
        libc::signal(SIGUSR1, handle_signals as usize);
    }
}

#[allow(dead_code)]
fn handle_signals(sig: i32) {
    register_signal_handler();

    let should_shutdown = match sig {
        SIGHUP => false,
        SIGALRM => false,
        SIGTERM => true,
        SIGQUIT => true,
        SIGUSR1 => true,
        _ => false,
    };

    unsafe {
        SHUTDOWN = should_shutdown;
    }

    return_early();
}

fn print_depth(depth: usize) {
    for _ in 0..depth {
        print!("#");
    }

    println!();
}

fn dive(depth: usize, max_depth: usize) {
    unsafe {
        if SHUTDOWN {
            println!("!");
            return;
        }
    }

    print_depth(depth);

    if depth >= max_depth {
        return;
    } else if depth == MOCK_SIGNAL_AT {
        unsafe {
            libc::raise(SIGUSR1);
        }
    } else {
        dive(depth + 1, max_depth);
    }
    print_depth(depth);
}
fn main() {
    const JUMP_SET: i32 = 0;
    register_signal_handler();

    let return_point = ptr_to_jmp_buf();

    let rc = unsafe { stjmp(return_point) };

    if rc == JUMP_SET {
        dive(0, 10);
    } else {
        println!("early return");
    }

    println!("finishing");
}
