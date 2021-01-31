use std::collections::HashMap;
// use std::fmt;
// use std::io;

// 使用as关键字，如同python
use std::fmt::Result;
use std::io::Result as IoResult;

use rand::Rng;

// 使用嵌套路径来清理大量使用的 use 语句
// use std::cmp::Ordering;
// use std::io;
// use std::io::Write;

// use std::{cmp::Ordering, io};
use std::io::{self, Write};

fn main() {
    println!("Hello, world!");
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// fn f1() -> fmt::Result{}

// fn f2() -> io::Result{}
