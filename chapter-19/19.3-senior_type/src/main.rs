/*
  高级类型相关主题：
  1. 适用类型别名创建同义词
    - Rust提供了类型别名功能，其主要用途便是减少代码字符重复
    - 使用type关键字
  2. Never类型 名为【!】的特殊类型：
    - 它没有任何值，在不返回的函数中充当返回类型
    - 不返回值的函数也被称为发散函数
    - never类型可被转换为任意类型
*/

type Kilometers = i32;

type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {}

fn returns_long_type() -> Thunk {
    Box::new(|| println!("hi"))
}

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    let f: Thunk = Box::new(|| println!("hi"));


    let guess = "";

    loop {
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    }
}
