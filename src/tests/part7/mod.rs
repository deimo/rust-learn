use crate::practice_linklist::cons_list::version1::ConsList;

#[cfg(test)]


#[test]
fn test0() {
    // let s = String::new().trim();
    // println!("s: ", s);
}


#[test]
fn test1() {
    let root: ConsList<'_> = ConsList::new();
    let first = root.head_insert(10);
    let second = first.head_insert(8);
    println!("{:?}", second);
    let third = second.head_insert(2);
    let list = third.head_insert(9);
    // println!("{:?}", first);
    // println!("{:?}", second);
    // println!("{:?}", list);
    let item8 = list.get_item(8);
    println!("{:?}", item8);
    println!("third: {:p}", &second);
    println!("item8: {:p}", &item8);

}

#[test]
fn test2() {
    #[derive(Debug)]
    struct Example<'a> {
        value: &'a String,
    }

    let example = Example {
        value: &String::from("hello"), // 错误：引用指向临时值
    };

    println!("{:?}", example.value)
}
