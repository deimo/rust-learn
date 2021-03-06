fn main() {
    // println!("Hello, world!");
    // another_function(5, 6);

    // let y = 5 + 6;

    /*
     * 演示表达式与语句：Rust面向表达式
     */
    // let x = 5;
    // let y = {
    //     let x = 1;
    //     x + 3
    // };
    // //## 语句的返回值是 () : 一个空的tuple;
    // println!("the x is {} ", x);
    // println!("the y is {} ", y);


    let x = plus_five(6);
    println!("the value of x is {}", x);
}

fn another_function(x: i32, y: i32) {
    // println!("Another function");
    println!("the value is {} ", x);
    println!("the value is {} ", y);
}

// Rust中函数的返回值：
// 在 -> 符号里边声明函数返回值的类型，但是不可以为返回值命名
// 在Rust中，通常返回值就是函数题里面最后一个表达式的值
// 如果需要提前返回，可以使用return关键字

fn plus_five(x: i32) -> i32 {
    x + 5
} 
