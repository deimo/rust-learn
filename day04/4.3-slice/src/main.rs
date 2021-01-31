fn main() {
    // 一个不安全的获取字符串索引示例
    // let mut s = String::from("hello world");
    // let wordIndex = first_world(&s);
    // s.clear();
    // println!("worldIndex: {}", wordIndex);

    /*
     * 字符串切片：
     * 指向字符串中一部分内容的引用
     *
     * 这里的切片语法表现与Python中切片语法表现一致
     * 语法：[(可选)..(可选)]
     *
     * 事实上，在Rust中，字符串字面值也就是切片
     */
    // let s = String::from("Hello World");
    // let hello = &s[0..5];
    // let world = &s[6..11];
    // println!("{}", hello);
    // println!("{}", world);
    /*
     * 几种特殊的切片
     */
    // 1.从头开始
    // println!("{}", &s[..5]);
    // // 2.到尾结束
    // println!("{}", &s[6..]);
    // // 3.从头到尾
    // println!("{}", &s[..]);

    // let mut s = String::from("hello world");
    // let word = first_world2(&s);
    // s.clear();
    // println!("worldIndex: {}", word);

    /*
     * 字符串字面值其实就是一个指向二进制程序特定位置的切片
     * 由于&str是不可变引用，所以字符串字面值也是不可变的！
     */
    // let s = "test";

    /*
     * 使用字符串切片作为函数的参数 
     */
     let my_string = String::from("Hello World");
     let word = first_world3(&my_string[..]);
     println!("word: {}", word);
     
     let my_string_literal = "I'm fine ";
     let word = first_world3(my_string_literal);
     println!("word: {}", word);

    // 其它数据类型的切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}

// 尝试解答
fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
// 使用字符串切片的方式来重写@first_world函数
fn first_world2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// 使用字符串切片作为参数，这样就可以同时接收String和&str类型的参数，使API的定义更加通用
fn first_world3(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
