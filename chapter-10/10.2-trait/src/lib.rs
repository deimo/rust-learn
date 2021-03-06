use std::fmt::{Debug, Display};

// trait的定义
pub trait Summary {
    // trait中定义方法
    fn summarize_author(&self) -> String;

    // trait中方法的默认实现
    fn summarize(&self) -> String {
        format!("Read more from {} ...", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

/*
 * 使用trait作为参数：
 * 1. impl trait语法
 * 2. trait bound 语法
 * 3. where 从句
 */ 
// pub fn notify(item: impl Summary) {
//     println!("breaking news! {}", item.summarize());
// }

// 使用trait bound语法来指定trait作为参数
pub fn notify1<T: Summary>(item: T) {
    println!("breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item1: T, item2: T) {}

// 通过 + 指定多个trait bound: 指定类型需要实现多个不同的trait
pub fn notify3(item: impl Summary + Display) {}

pub fn notify4<T: Summary + Display>(item: T) {}

// 复杂函数签名下的trait bound，反面示例
pub fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    1
}

// 通过where简化trait bound
pub fn some_function2<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Display + Debug,
{
    1
}

/*
 * 返回实现了trait类型的函数
 *
 */
fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably aleady know , people"),
        reply: false,
        retweet: false,
    }
}