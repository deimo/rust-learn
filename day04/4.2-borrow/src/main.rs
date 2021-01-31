// fn main() {
    /*
     * 1. 演示引用的使用
     * 引用：允许程序员引用某些值而不去取得其所有权，默认不可变
     * 借用：将引用作为函数参数这种代码行为称为借用
     */
    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);   // 没有转移所有权，而是传递了引用
    // println!("len1: The length of '{}' is {}", s1, len);
    

    /*
     * 2. 演示可变引用的例子
     */
    // let mut s1 = String::from("Hello");
    // let len = calculate_length(&mut s1);
    // println!("The length if '{}' is {}", s1, len);

    /*
     * 演示可变引用的限制1
     */
    //  let mut s = String::from("Hello");
    //  // TODO: 你可以声明多个可变引用，但是一旦使用，就会报错
    //  let s1 = &mut s;
    //  let s2 = &mut s;
    //  println!("the length of '{}' is", s1);

    /*
     * 演示可变引用的限制1-1 
     * 通过创建新的作用域，来允许非同时的创建多个可变引用
     */
    // let mut s = String::from("Hello");
    // {
    //     let s1 = &mut s;
    // }
    // let s2 = &mut s;
    // println!("{}", s2);

    /*
     * 演示可变引用的限制2
     */
    //  let mut s = String::from("Hello");
    //  let r1 = &s;
    //  let r2 = &s;
    //  let s1 = &mut s;
    //  // TODO: 你可以声明可变引用和不可变引用，但是一旦使用，就会报错
    //  println!("{} {} {}", r1, r2, s1);

// }

// 引用没有获得变量的所有权
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }



// 可变引用的例子
/*
 * 可变引用的重要限制： 
 * 1. 在特定作用域内，对某一块数据，只能有一个可变引用，其目的是为了避免数据竞争
 * 2. 不可以同时拥有一个可变引用和不可变引用
 */


fn calculate_length(s: &mut String) -> usize {
    s.push_str(", World");
    s.len()
}

/*
 * Dangling Reference 悬空指针：
 * Rust编译器保证了引用永远都不是悬空引用: 
 * 编译器会保证在引用离开作用域之前数据不会离开作用域
 */
 
 fn main () {
    let r = dangle();
 }

 fn dangle() -> &String {
     // 编译无法通过，因为变量s在其作用域之外将会被drop，导致出现悬空指针，Rust不允许这种情况的出现 
     let s = String::from("hello");
     &s
 }