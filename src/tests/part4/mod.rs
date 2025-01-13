pub mod drop_demo;
pub mod rc_demo;

pub mod cell_demo;

use std::ops::Deref;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Person { name, age }
    }

    fn display(self: &mut Person, age: u8) {
        let temp = &self;
    }
}

#[cfg(test)]
#[test]
fn test1() {
    let x = 5;
    let y = &x;

    let a = Box::new(1);
    let sum = *a + 1;
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn test2() {
    let y = MyBox::new(5);
    println!("{}", *y);
}

fn display(s: &str) {
    println!("{}", s);
}

#[test]
fn test3() {
    let s = MyBox::new(String::from("hello world"));
    display(&s);

    let s1: &str = &s;
    let s2 = s.to_string();
}
