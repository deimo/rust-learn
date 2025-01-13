use core::num;
#[cfg(test)]
use std::cell::Cell;
use std::{cell::RefCell, rc::Rc};

#[test]
fn test1() {
    let c = Cell::new("asdf");
    println!("before set c: {:?}", c);
    let one = c.get();
    c.set("qwer");
    println!("after set c: {:?}", c);

    let two = c.get();
    println!("{}, {}", one, two);
}

#[test]
fn test3() {
    let c = Cell::new(String::from("asdf"));
    // println!("before set c: {:?}", c);
}

#[test]
fn test2() {
    let mut s = "hello";
    s.to_string().push_str("world");
    println!("s: {}", s);

    let mut s2 = String::from("Hello");
    s2.push_str(" world");
    println!("s2: {}", s2);
}

#[test]
fn main() {
    let s = RefCell::new(String::from("hello, world"));
    // let s1 = s.borrow();
    let mut s2 = s.borrow_mut();
    // println!("s1 = {}, s2 = {}", s1, s2);
    s2.push_str(" hhhhhh");
    println!("s2: {}", s2);
}

trait Messenger {
    fn send(&self, msg: String);
}

struct MsgQueue {
    msg_cache: RefCell<Vec<String>>,
}

impl Messenger for MsgQueue {
    fn send(&self, msg: String) {
        self.msg_cache.borrow_mut().push(msg);
    }
}

#[test]
fn test5() {
    let mq = MsgQueue {
        msg_cache: RefCell::new(Vec::new()),
    };
    mq.send("hello, world".to_string());
}

#[test]
fn test6() {
    let s = Rc::new(RefCell::new("我很善变，有多个主人".to_string()));
    let s1 = s.clone();
    let s2 = s.clone();

    s2.borrow_mut().push_str("oh yeah!");
    println!("{:?}\n {:?}\n {:?}", s, s1, s2);
}

fn is_even(i: i32) -> bool {
    i % 2 == 0
}

fn retain_even(nums: &mut Vec<i32>) {
    use std::cell::Cell;
    // let mut i = 0;
    // for num in nums.iter().filter(|&num| is_even(*num)) {
    //     nums[i] = *num;
    //     i += 1;
    // }
    // nums.truncate(i);
    let slice = Cell::from_mut(&mut nums[..]).as_slice_of_cells();

    let mut i = 0 ;
    for num in slice.iter().filter(|num| is_even(num.get())) {
        slice[i].set(num.get());
        i += 1;
    }
    nums.truncate(i);
}

#[test]
fn test7() {
    let mut i = 0;
}
