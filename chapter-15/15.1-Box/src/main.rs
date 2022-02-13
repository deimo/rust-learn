/// Drop Trait

/*
    实现Drop Trait，可以让我们自定义当值将要离开作用域时发生的动作
    - 例如：文件，网络资源释放等
    - 任何类型都可以实现Drop trait
    - 不可以手动调用Drop trait中的 drop 函数，但是可以使用 std::mem::drop函数，来提前drop值
*/

use std::mem::drop;
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}` ", self.data);
    }
}

fn main() { 
    let c = CustomSmartPointer {data: String::from("my stuff")};
    drop(c);
    let d = CustomSmartPointer{data: String::from("other stuff")};
    println!("CustomSmartPointers created. ");
}