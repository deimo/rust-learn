#[cfg(test)]
use crate::part5_special_refrence::List::{Cons, Nil};
use crate::part5_special_refrence::WhatAboutThis;
use std::{cell::RefCell, rc::Rc};

#[test]
fn test1() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a的初始化rc计数 = {}", Rc::strong_count(&a));
    println!("a指向的节点 = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("在b创建后，a的rc计数 = {}", Rc::strong_count(&a));
    println!("b的初始化rc计数 = {}", Rc::strong_count(&b));
    println!("b指向的节点 = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("在更改a后，b的rc计数 = {}", Rc::strong_count(&b));
    println!("在更改a后，a的rc计数 = {}", Rc::strong_count(&a));

    // 下面一行println!将导致循环引用
    // 我们可怜的8MB大小的main线程栈空间将被它冲垮，最终造成栈溢出
    // println!("a next item = {:?}", a.tail());
}

#[test]
fn test2() {
    let five = Rc::new(5);
    let weak_five = Rc::downgrade(&five);

    let strong_five = weak_five.upgrade();
    assert_eq!(*strong_five.unwrap(), 5);

    drop(five);
    let strong_five = weak_five.upgrade();
    assert_eq!(strong_five, None);
}

struct SelfRef<'a> {
    value: String,
    pointer_to_value: &'a str,
}

#[test]
fn test3() {
    let s = "aaa".to_string();
    // let v = SelfRef {
    //     value: s,
    //     pointer_to_value: &s
    // };
}

#[test]
fn test4() {
    let mut tricky = WhatAboutThis {
        name: "Annablelle".to_string(),
        nickname: None,
    };

    tricky.nickname = Some(&tricky.name[..4]);
    println!("{:?}", tricky);
}

// fn creator<'a>() -> WahtAboutThis<'a> {
//     let mut tricky = WahtAboutThis {
//         name: "Annabelle".to_string(),
//         nickname: None,
//     };
//     tricky.nickname = Some(&tricky.name[..4]);
//     tricky
// }

#[test]
fn test5() {
    let mut tricky = WhatAboutThis {
        name: "Annabelle".to_string(),
        nickname: None,
    };
    tricky.tie_the_knot();

    // println!("{:?}", tricky);
}

#[test]
fn test6() {
    let mut s = String::from("hello");
    println!("{:?}", s); // 不需要可变借用
    s.push_str(" world"); // 可变操作仍然可行
    println!("{:?}", s);
}
