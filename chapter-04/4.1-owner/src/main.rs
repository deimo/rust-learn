/* 演示值的作用域
    fn main() {
        // s 不可用
        let s = "hello";  // s 可用
                        // 可以对s进行操作
    } // s 作用域到此结束，s 不再可用 
*/

// fn main() {
//     let mut s = String::from("hello");
//     s.push_str(", World");
//     println!("{}", s);
// }

/*演示所有权的移动   
// fn main() {
//     let s1 = String::from("Hello");
//     let s2 = s1;
//     println!("{}", s2);
//     println!("{}", s1);
// }
*/

// 演示clone
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();
//     println!("{}, {}", s1, s2);
// }

// 演示所有权在函数下影响：移动
// fn main() {
//     let s = String::from("test");
//     take_ownership(s);
//     let x = 4;
//     makes_copy(x);
//     println!("x is {}", x);
// }

// 演示函数返回值所有权的移动
// fn main() {
//     let s1 = gives_ownership();
//     println!("s1 is {}", s1);

//     let s2 = String::from("hello");
//     println!("s2 is {} in line 45", s2);

//     let s3 = takes_and_gives_back(s2);
//     // FIXME: 报错，因为此时值的所有权已移交给了s3
//     // println!("s2 is {} in line 48", s2);

// }

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_number: i32) {
    println!("{}", some_number);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}


fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}


fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("the length of '{}' is {}.", s2, len);
}