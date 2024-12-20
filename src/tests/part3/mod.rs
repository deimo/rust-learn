pub mod demo1;

use std::convert::TryInto;
use std::mem;

#[cfg(test)]
#[test]
pub fn demo() {
    let a: i32 = 10;
    let b: u16 = 11;
    if a < b as i32 {
        println!(">>>>>");
    }
}

#[test]
pub fn demo2() {
    let a = 3.1 as i8;
    let b = 100_i8 as i32;
    let c = 'a' as u8;
    println!("{}, {}, {}", a, b, c);
}

#[test]
pub fn demo3() {
    let mut values: [i32; 2] = [1, 2];
    let p1 = values.as_mut_ptr();
    let first_address = p1 as usize;
    let second_address: usize = first_address + mem::size_of::<i32>();
    let p2 = second_address as *mut i32;
    unsafe {
        *p2 += 1;
    }
    println!("values: {:?}", values);
}

#[test]
pub fn demo4() {
    let a: u8 = 10;
    let b: u16 = 1500;

    match b.try_into() {
        Ok(b_) => {
            if a < b_ {
                println!(">>>>")
            }
        }
        Err(_) => {
            println!("fail")
        }
    }
}

#[test]
pub fn demo5() {
    let b: i16 = 1500;

    let b_: u8 = match b.try_into() {
        Ok(b1) => b1,
        Err(e) => {
            println!("{:?}", e.to_string());
            0
        }
    };
}
