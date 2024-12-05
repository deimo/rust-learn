//! クロージャ学習用モジュール

mod three_fn;
/**
 * 一番簡単なサンプルです
 */
pub fn demo1() -> i32 {
    let x = 1;
    let sum = |y| x + y;
    sum(2)
}

/**
 * `demo2` 型推論の悪用サンプル
 */
pub fn demo2() {
    // let example_closure = |x| x;
    // let s = example_closure(String::from("hello"));
    // let n = example_closure(5);
}

/// 構造体にクロージャ使うサンプル
pub mod struct_demo {
    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        query: T,
        value: Option<u32>,
    }
    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(query: T) -> Cacher<T> {
            Cacher { query, value: None }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.query)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }
}

/// 各種Fnトレイトサンプル
pub mod fn_trait_demo {
    /*
    FnOnceの悪用サンプル
    */
    pub fn fn_once<F>(func: F)
    where
        F: FnOnce(usize) -> bool + Copy,
    {
        println!("{}", func(3));
        println!("{}", func(4));
    }
    /*
    Fnトレイト サンプル
     */
    pub fn fn_demo() {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {list:?}");

        let only_borrows = || println!("From closure: {list:?}");

        println!("Before calling closure: {list:?}");
        only_borrows();
        println!("After calling closure: {list:?}");
    }

    pub fn fn_mut() {
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {list:?}");

        let mut borrows_mutably = || list.push(6);
        borrows_mutably();
        println!("After calling closure: {list:?}");
    }
}
