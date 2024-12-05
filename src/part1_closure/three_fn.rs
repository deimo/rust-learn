pub fn test_fn(data: String) {
    let fn_closure = |val| println!("{}", val);
    fn_closure(data);
}

pub fn test_fn_mut(data: &mut String) {
    let mut fnmut_closure = |val| println!("{:?}", data.push_str(val));
    fnmut_closure("hhhh");
}

pub fn test_fn_once(data: String) {
    let fnonce_closure = |val| {
        println!("val: {}", val);
        drop(data);
    };
    fnonce_closure("data");
}

pub fn foo() {
    // 拿所有权
    let s = String::new();
    let update_string = move || println!("{}", s);

    exec(update_string);
    // exec(update_string);
}

pub fn bar() {
    // 可变引用
    let mut s = String::new();
    let mut update_string = || s.push_str("hello");
    exec(update_string);
    // exec(update_string);
}

pub fn baz() {
    let s = String::new();
    let update_string = move || println!("{}", s);
    exec(update_string);
}

pub fn demo() {
    let s = String::new();

    let update_string = || println!("{}", s);

    exec(update_string);
    exec1(update_string);
    exec2(update_string);
}

pub fn demo2() {
    let mut s = String::new();
    let update_string = |str| -> String {
        s.push_str(str);
        s
    };
    exec4(update_string);
}

fn exec<F: FnOnce()>(f: F) {
    f()
}

fn exec1<F: FnMut()>(mut f: F) {
    f()
}

fn exec2<F: Fn()>(f: F) {
    f()
}

fn exec4<'a, F: FnOnce(&'a str) -> String>(mut f: F) {
    f("hello");
}

fn factory(x: i32) -> impl Fn(i32) -> i32 {
    let num = 5;

    move |x| x + num
}


pub fn test() {
    let mut s = String::new();
    let update_string =  |str| s.push_str(str);
    exec5(update_string);
    println!("{:?}",s);
}

fn exec5<'a, F: FnMut(&'a str)>(mut f: F)  {
    f("hello")
}
