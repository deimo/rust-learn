use std::process::id;
use std::slice;

use std::{
    env::consts,
    slice::from_raw_parts,
    str::{self, from_utf8_unchecked},
    string,
};

#[cfg(test)]
#[test]
fn test1() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
}

#[test]
fn test2() {
    let mut num = 5;
    let r1 = &num as *const i32;
    unsafe {
        println!("r1 is {}", *r1);
    }
}

#[test]
fn test3() {
    let address = 0x012345usize;
    let r = address as *const i32;
}

fn get_memory_location() -> (usize, usize) {
    let string = "hello world";
    let pointer = string.as_ptr() as usize;
    println!("pointer: {}", pointer);
    let length = string.len();
    (pointer, length)
}

fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
}

#[test]
fn test4() {
    let (pointer, length) = get_memory_location();
    // let message = get_str_at_location(pointer, length);
    let message = get_str_at_location(1000, 10);
    println!(
        "the {} bytes at 0x{:x} stored: {}",
        length, pointer, message
    );
}

#[test]
fn test5() {
    let a = 1;
    let b = &a as *const i32;
    let c: *const i32 = &a;
    unsafe {
        println!("{}", *c);
    }
}

#[test]
fn test6() {
    let a = Box::new(10);
    let b: *const i32 = &*a;
    let b2 = &*a as *const i32;
    println!("{}", a);
    unsafe {
        println!("{}", *b);
        println!("{}", *b2);
    }
    let c = Box::into_raw(a);
    unsafe {
        println!("{}", *c);
    }
}

unsafe fn dangerous() {}

#[test]
fn test7() {
    unsafe {
        dangerous();
    }
}

fn splite_at_mut(items: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = items.len();
    let ptr = items.as_mut_ptr();

    assert!(mid <= len);
    // (&mut slice[..mid], &mut slice[mid..])
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
