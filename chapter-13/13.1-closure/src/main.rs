fn main() {
    let x = 4;

    // let equal_to_x = |z| {
    //     z == x 
    // };

    fn eqaul_to_x_fn(z: i32) -> bool {
        z == x
    }

    let y = 4;
    // assert!(equal_to_x(y))
    assert!(eqaul_to_x_fn(y))

    // 取得所有权: FnOnce
    // 可变借用: FnMut
    // 不可变借用: Fn

    /* 
      - 所有的闭包都实现了FnOnce
      - 没有移动捕获变量的实现了FnMut
      - 无需可变访问捕获变量的闭包实现了Fn
    */
}
