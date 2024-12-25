use std::sync::Arc;

pub struct Foo {
    pub x: u32,
    pub y: u16,
}

pub struct Bar {
    pub a: u32,
    pub b: u16,
}

fn reinterpret(foo: Foo) -> Bar {
    let Foo { x, y } = foo;
    Bar { a: x, b: y }
}

#[test]
pub fn demo1() {
    let foo = Foo { x: 1, y: 2 };
    let bar = reinterpret(foo);
}

fn do_stuff<T: Clone>(value: &T) {
    let cloned = value.clone();
}

fn do_stuff2<T>(value: &T) {
    let cloned = value.clone();
}

#[derive(Clone)]
struct Container<T>(Arc<T>);

fn clone_containers<T>(foo: &Container<i32>, bar: &Container<T>) {
    let foo_cloned = foo.clone();
    let bar_cloned = bar.clone();
}

fn demo2(s: Box<&str>) {
    s.len();

    let s1: Box<&str> = "hello".into();
}

enum MyEnum {
    A = 1,
    B,
    C,
}

impl TryFrom<i32> for MyEnum {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == MyEnum::A as i32 => Ok(MyEnum::A),
            x if x == MyEnum::B as i32 => Ok(MyEnum::B),
            x if x == MyEnum::C as i32 => Ok(MyEnum::C),
            _ => Err(()),
        }
    }
}

#[test]
fn demo3() {
    let x = MyEnum::C as i32;
    println!("x = {}", x);
    // 将整数转换为枚举，失败
    // match x {
    //     MyEnum::A => {}
    //     MyEnum::B => {}
    //     MyEnum::C => {}
    //     _ => {}
    // }
}

#[test]
fn test4() {
    let arr = [0; 1000];
    let arr1 = arr;

    println!("the arr's len is: {:?}", arr.len());
    println!("the arr1's len is: {:?}", arr1.len());

    // let s1 = String::from("value");
    // println!("s1.len(): {}", s1.len());
    // let s2 = s1;
    // println!("s1.len(): {}", s1.len());
    // println!("s2.len(): {}", s2.len());

    // let arr = Box::new([0; 1000]);
    // let arr1 = arr;
    // println!("arr1.len(): {}", arr1.len());
    // println!("arr.len(): {}", arr.len());
}

#[test]
fn test5() {
    let arr = vec![Box::new(1), Box::new(2)];
    // let first = arr[1];
    let arr_ = arr;
    println!("arr.len: {}", arr.len());
    println!("arr_.len: {}", arr_.len());


    let arr2 = vec![String::from("hhhh"), String::from("aaa")];
    let s = arr2[0];
    let s2 = String::from("sss");

    let arr3 = vec!["aaa", "bbb"];
    arr3[0];

    let items = vec![1, 2, 3];
    let item = items[0];
}
