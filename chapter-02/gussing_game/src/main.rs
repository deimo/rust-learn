// prelude
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("猜数!");
    let secrect_number = rand::thread_rng().gen_range(1, 101);
    println!("神秘数字是{} ", secrect_number);
    loop {
        println!("猜测一个数!!");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取");
        println!("你猜测的数是：{}", guess);
        // shdow 隐藏变量    --> 常见于类型转换
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入一个数");
                continue;
            }
        };
        match guess.cmp(&secrect_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
