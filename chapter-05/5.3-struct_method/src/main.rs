#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,

    
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.length > other.length
    }

    fn get_width(self) -> u32 {
        self.width
    }
    
    fn get_length(&mut self) -> u32 {
        self.length
    }

    // 关联函数
    fn square(size:u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }
    
}


fn main() {
    let s = Rectangle::square(20);
    println!("Hello, world!");
    //# struct方法
    /*
     * 方法与函数类似：fn关键字，名称，参数，返回值
     * 不同之处
     *  - 方法是在struct（或enum，trait对象）的上下文中定义
     *  - 第一个参数总是self,表示方法被调用的struct实例
     * 
     * 特别地：
     * Rust在调用方法时会自动引用或者解引用
     * 这样的话，无论方法定义的第一个参数是否需要所有权都可以同样的语法形式进行调用
     * 以上述Rectangle为例，无论参数[&self]还是[self]或者[&mut self]
     * 都可以一种形式进行调用
     * rect.xx() <==> &rect.xx()  <==> &mut rect.xx()
     * 
     */
    let mut rect1 = Rectangle {
        width: 30,
        length: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        length: 40,
    };
    let rect3 = Rectangle {
        width: 35,
        length: 35,
    };
    // println!("{}", rect1.area());
    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));
    println!("test is {} ", &mut rect1.get_length());
    println!("length is {}", rect1.get_length());
    println!("width {}", rect1.get_width())
}
