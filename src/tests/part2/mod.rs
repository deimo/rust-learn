use std::collections::HashMap;

#[cfg(test)]
#[test]
fn demo1() {
    for i in 1..10 {
        println!("{}", i);
    }
}

#[test]
fn demo2() {
    let arr = [1, 2, 3];
    for v in arr.into_iter() {
        println!("{}", v);
    }
}

#[test]
fn demo3() {
    let arr = [1, 2, 3];
    let mut arr_iter = arr.into_iter();
    // let next = arr_iter.next();
    assert_eq!(arr_iter.next(), Some(1));
    assert_eq!(arr_iter.next(), Some(2));
    assert_eq!(arr_iter.next(), Some(3));
    assert_eq!(arr_iter.next(), None);
}

#[test]
fn demo4() {
    let values = vec![1, 2, 3];
    {
        let result = match IntoIterator::into_iter(values) {
            mut iter => loop {
                match iter.next() {
                    Some(x) => {
                        println!("{}", x);
                    }
                    None => break,
                }
            },
        };
        result
    }
}

#[test]
fn demo5() {
    let values = vec![1, 2, 3];

    for v in values.into_iter().into_iter().into_iter() {
        println!("{}", v);
    }
}

#[test]
fn demo6() {
    let values = vec![1, 2, 3];
    for v in values.into_iter() {
        println!("{}", v);
    }
    // println!("{:?}", values);

    let values = vec![1, 2, 3];
    let _values_iter = values.iter();
    println!("{:?}", values);

    let mut values = vec![1, 2, 3];
    let mut values_iter_mut = values.iter_mut();
    if let Some(v) = values_iter_mut.next() {
        *v = 0;
    }
    println!("{:?}", values);
}

#[test]
fn demo7() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);

    println!("{:?}", v1);
    // println!("{:?}", v1_iter);
}

#[test]
fn demo8() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v2)
}

#[test]
fn demo9() {
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];
    let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();
    println!("{:?}", folks);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn demo10() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
