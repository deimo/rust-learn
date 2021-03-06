fn main() {
    /*
     * 演示Rust的分支控制
     */
    // let number = 7;
    // if number < 5 {
    //     println!("condition was true");
    // }else {
    //     println!("condistion was false");
    // }

    // 此处代码最好使用match重构
    // let num = 6;
    // if num % 4 == 0 {
    //     println!("number is divisble by 4");
    // } else if num % 3 == 0 {
    //     println!("number is divisble by 3");
    // } else if num % 2 == 0 {
    //     println!("number is divisble by 2");
    // } else {
    //     println!("nnumber is not divisible by 4, 3 or 2");
    // }
    // let condition = true;

    // //注意 if是一个表达式，所以可以将其用为右值表达式
    // let x = if condition { 5 } else { 6 };
    // println!("the value of x is {}", x);

    // 演示Rust的循环控制: loop, while , for
    // loop循环;
    // loop {
    //     println!("again");
    // }
    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("the result is: {}", result);

    // while循环
    // let mut number = 3;
    // while number != 0 {
    //     println!("{}!", number) ;
    //     number -= 1;
    // }
    // println!("LIFTOFF!!!");

    // for循环
    // let a = [10, 20, 30, 40, 50];
    // for element in a.iter() {
    //     println!("the value is: {}", element);
    // }
    
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF");

}
