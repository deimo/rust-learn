/// 模式匹配符号：match
/// 允许一个值与一系列模式进行匹配，并执行匹配的模式对应的代码
/// 模式可以是字面值，变量名，通配符
/// match匹配时必须穷举所有的可能性！
/// _ 通配符：代表剩下的所有模式

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("penny");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
/// 匹配的分支可以绑定到被匹配对象的部分值：可以从enum变体中提取值
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        /// 绑定值的模式匹配
        Coin::Quarter(state) => {
            println!("State quarter fron {:?}", state);
            25
        }
    }
}

fn main() {
    // println!("Hello, world!");
    // let c = Coin::Quarter(UsState::Alaska);
    // println!("{}", value_in_cents(c));

    //     let five = Some(5);
    //     let six = plus_one(five);
    //     let none = plus_one(None);

    // 通配符演示实例
    let v = 11u8;
    match v {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
    println!("{}" , v);
}

/// 匹配Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
