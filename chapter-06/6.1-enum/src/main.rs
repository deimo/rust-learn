/// 定义枚举
/// 枚举允许我们列举所有可能的值来定义一个类型

enum IpAddrKind {
    V4,
    V6,
}

//# 将枚举值作为结构体的字段类型
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

//# 将数据直接附加到枚举的变体中
// enum IpAddr {
//     V4(String),
//     V6(String),
// }

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}


enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {
    //# 创建枚举值
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
    // route(four);
    // route(six);
    // route(IpAddrKind::V6);

    //# 枚举类型作为结构体字段类型
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address : String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //      kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // let home = IpAddr::V4(127, 0,0, 1);
    // let loopback = IpAddr::V6(String::from("::1"));

    let q = Message::Quit;
    let m = Message::Move {x:12, y: 24};
    let w = Message::Write(String::from("Hello"));
    let c = Message::ChangeColor(0, 255, 255);
    m.call();
    
}

//# 接收枚举类型作为参数
fn route(ip_kind: IpAddrKind) {}
