/*
  高级Trait
  1. 关联类型：Trait中的类型占位符，它可以用于Trait的方法签名中
     - 可以定义出包含某些类型的Trait，而在实现前无需知道这些类型时什么
    特点：
        - 实现Trait时无需标注泛型
        - 无法为单个类型多次实现某个Trait

  2. 默认泛型参数与运算符重载：可以在使用泛型参数时为泛型指定一个默认的具体类型

  3. 完全限定语法(调用同名方法)
     只有当Rust无法区分期望调用哪个具体的实现，才需使用此语法
     语法：<Type as Trait>::function(args1, args2 ...)

  4. 使用supertrait 来要求trait附带其它trait的功能
    需要在一个trait中使用其它trait的功能：
      - 需要被依赖的trait也被实现
      - 那个被间接依赖的trait就是当前trait的supertrait
     语法：[自定义trait]: [被依赖trait]

  5. 使用newType模式在外部类型上实现外部trait
     孤儿规则：只有当trait或类型定义在本地包是，才能为该类型实现这个trait
     可以通过newtype模式来绕过这一规则
       - 利用tuple struct（元组结构体）创建一个新的类型
*/

/* 一个使用newType规则为外部Vec实现外部Display trait */
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

pub trait Iterator {
    type Item; // -> 关联类型
    fn next(&mut self) -> Option<Self::Item>;
}

use core::fmt;
use std::ops::Add;

struct Millimeter(u32);
struct Meters(u32);

impl Add<Meters> for Millimeter {
    type Output = Millimeter;

    fn add(self, other: Meters) -> Millimeter {
        Millimeter(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

/// 一个
struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is you captain speaking");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up ")
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

pub trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("pubby")
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);

    // 一个使用完全限定于法的例子
    println!("A baby dog is called a {}", Dog::baby_name());

    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
