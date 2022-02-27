/*
  高级函数与闭包
  1. 函数指针(function pointer)
      - 可以将函数传递给其它函数
      - 函数在传递过程中会被强制转换成fn类型
      fn 是一个类型，而不是trait
      函数指针实现了全部的3重闭包trait（Fn，FnMut，FnOnce）：
          - 总是可以把函数指针用作参数传递给一个接收闭包的函数
          - 倾向于搭配闭包trait的泛型来编写函数：可以同时接收闭包和普通函数
  2. 返回闭包
     - 闭包使用trait进行表达，无法在函数中直接返回一个闭包，可以将一个实现了该trait的具体类型作为返回值
*/

// !错误示范
// fn returns_closure() -> Fn(i32) -> i32 {
//     |x| x + 1
// }

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}




fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    // 例1：
    let answer = do_twice(add_one, 5);
    println!("the answer is: {}", answer);

    // 例2：
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    // 例3：
    enum Status {
        Value(u32),
        Stop,
    }
    impl Status {
        fn run(&self) {
            println!("fn rub")
        }
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}
