/// 在模式中忽略值

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

enum Message {
    Hello {id: i32},
}
fn main() {
    /* 1. 使用_忽略整个值 */
    // foo(3, 4);

    /* 2. 使用嵌套_忽略值的一部分 --> 只关注类型 */
    // let mut setting_value = Some(5);
    // let new_setting_value = Some(10);
    // match (setting_value, new_setting_value) {
    //     (Some(_), Some(_)) => {
    //         println!("can't overwriter an existing customized value")
    //     }
    //     _ => {
    //         setting_value = new_setting_value;
    //     }
    // }
    // println!("setting is {:?}", setting_value);
    // let numbers = (2, 4, 6, 8, 16);

    // match numbers {
    //     (first, _, third, _, fifth) => {
    //         println!("Some numbers: {}, {}, {}", first, third, fifth)
    //     }
    // }

    /* 3. 使用_开头命名忽略未使用的变量 */
    // let _x = 5;
    // let y = 10;

    // let s = Some(String::from("hello"));
    // if let Some(_) = s {  // 只有下划线时不会发生绑定动作
    //     println!("found a String")
    // }
    // println!("{:?}", s);

    /* 4. 使用..来忽略值的剩余部分 */
    // let origin = Point {x: 0, y: 0, z: 0};
    // match origin {
    // 只需要字段x的值
    //     Point { x, ..} => println!("x is {}", x),
    // }
    // let numbers = (2, 4, 6, 8, 32);
    // match numbers {
    //     (first, .., last) => {
    //         println!("Some numbers: {}, {}", first, last);
    //     }
    // }

    /* 5. 使用match守卫来提供额外的条件
        - match守卫就是match arm分支模式后额外的if条件，想要匹配的该条件也必须满足
        - match守卫使用于比单独的模式更复杂的场景
    */
    // let num = Some(4);
    // match num {
    //     Some(x) if x < 5 => {
    //         println!("less than five: {}", x);
    //     }
    //     Some(x) => println!("{}", x),
    //     None => (),
    // }

    // let x = 4;
    // let y = false;
    // match x {
    //     4 | 5 | 6 if y => println!("yes!"),
    //     _ => println!("no"),
    // }

    /* 5. @绑定
        @ 符号可以创建一个变量，该变量可以在测试某个值是否与模式匹配的同时保存该值
    */
    let msg = Message::Hello {id: 5};

    match msg {
        Message::Hello {
            id: id_variable @ 3..= 7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello {id: 10..=12} => {
            println!("Found an id in another range")
        }

        Message::Hello {id} => {
            println!("Found some other id: {}", id)
        }
    }
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y)
}
