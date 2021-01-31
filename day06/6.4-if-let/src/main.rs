/// if let简介
/// 处理只关心一种匹配而忽略其它匹配的情况
/// 注意：使用 if let 就相当于放弃了穷举的可能!
fn main() {
    // println!("Hello, world!");
    let v = Some(0u8);
    // match v {
    //     Some(3) => println!("three"),
    //     _ => (),
    // }

    /// if let 是上述模式匹配的简写形式
    // if let Some(3) = v {
    //     println!("three");
    // }



    /// if let 搭配 else 使用
    // match v {
    //     Some(3) => println!("three"),
    //     _ => println!("others"),
    // }
    
    if let Some(3) = v {
        println!("three");
    } else {
        println!("others");
    }
}
