use rust_learn::part1_closure;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    // let mut list = [
    //     Rectangle {
    //         width: 10,
    //         height: 1,
    //     },
    //     Rectangle {
    //         width: 3,
    //         height: 5,
    //     },
    //     Rectangle {
    //         width: 7,
    //         height: 12,
    //     },
    // ];
    // list.sort_by_key(|r| r.width);
    // println!("{:#?}", list);
    // println!("{list:#?}");

    // let mut list = [
    //     Rectangle { width: 10, height: 1 },
    //     Rectangle { width: 3, height: 5 },
    //     Rectangle { width: 7, height: 12 },
    // ];

    // let mut sort_operations = vec![];
    // let value = String::from("closure called");

    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });
    // println!("{list:#?}");

    let mut s = String::new();
    let update_string = |str| s.push_str(str);
    exec(update_string);

    println!("{:?}", s);
}

fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
    f("hello")
}
