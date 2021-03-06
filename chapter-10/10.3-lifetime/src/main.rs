use std::fmt::Display;

/*
 * struct中的生命周期
 */
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("call me Ishmael. Some years ago ...");
    let first_sentence = novel.split(".").next().expect("could not foud a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 指定生命周期参数的方式依赖于函数功能：若明确返回其中一个参数，则不再需要标注其它参数的生命周期
fn longest1<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// 返回函数内部的值的正确操作：不返回引用，直接将所有权返回
fn longest2<'a>(x: &'a str, y: &'a str) -> String {
    let result = String::from("abc");
    result
}

/**
 * 函数/方法的参数：输入生命周期
 * 函数/方法的返回值：输出生命周期
 *
 * 生命周期省略的3个规则
 * - 规则1应用于输入生命周期
 * - 规则2，3应用于输出生命周期
 *   ◆　每个引用类型的参数都有自己的生命周期
 *   ◆　如果只有1个输入生命周期参数，那么该生命周期会被赋给所有的输出生命周期参数
 *   ◆　如果有多个输入生命周期参数，但其中一个是&self或&mut self(是方法)，那么self的生命周期会被赋给所有的输出生命周期参数
 *
 * 静态生命周期：'static，在整个程序的执行期间引用都有效
 * - 所有的字符串字面值都拥有静态生命周期
 *
 */
fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
