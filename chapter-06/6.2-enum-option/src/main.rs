fn main() {
    let some_number = Some(5);
    let value = some_number.expect("no value");
    println!("value is {}", value);
    // let some_string = Some("a string");
    // 如果使用None而不是some，需要告诉Rust Option<T>是什么类型
    let absent_number: Option<i32> = None;
    let absent = absent_number.expect("no value!!!!");
    println!("the absent is {}", absent);
    
    let x : i8 = 5;
    let y : Option<i8> = Some(5);
    
    // let sum = x + y;
}