/*
为了支持多重所有权，Rust提供了RC<T>
RC<T>: 引用计数智能指针
       用于处理一个值有多个所有者的情形，比如图数据结构
工作形式：
    1. 追踪所有值的引用
    2. 发现为0个引用：该值可以被清理
※ RC<T> 只能用于单线程！
*/

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
fn main() {
    // 例1： 一个简单的使用Rc引用生成多指向链表的例子
    // let a = Rc::new(Cons(5, 
    //                     Rc::new(Cons(10, 
    //                         Rc::new(Nil)))));
    // let b = Cons(3, Rc::clone(&a));
    // let c = Cons(4, Rc::clone(&a));

    let a = Rc::new(Cons(5,
                                Rc::new(Cons(10, 
                                    Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
