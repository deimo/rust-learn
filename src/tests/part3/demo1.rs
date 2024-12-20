
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
    println!(" -----");
}
