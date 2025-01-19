/// consList的引用版本，会出现temporary value dropped while borrowed难以解决
pub mod version1 {
    #[derive(Debug, Clone)]
    pub enum ConsList<'a> {
        Cons(i32, &'a ConsList<'a>),
        Nil,
    }

    impl<'a> ConsList<'a> {
        // 得到一个新的ConsList
        pub fn new() -> ConsList<'a> {
            ConsList::Nil
        }

        // 头插法：造成了大量的空间的浪费
        pub fn head_insert(&self, item: i32) -> ConsList {
            ConsList::Cons(item, &self)
        }

        // 尾插法
        fn tail_insert() {
            todo!()
        }

        pub fn get_item(&self, target: i32) -> ConsList {
            match self {
                ConsList::Cons(value, next) => {
                    if *value == target {
                        Clone::clone(&self)
                    } else {
                        // TODO:这里又出现了奇怪的deref现象，声明为&，但在使用时却可以直接使用&&
                        next.get_item(target)
                    }
                }
                ConsList::Nil => ConsList::Nil,
            }
        }

        pub fn remove(&self, target: i32) -> ConsList {
            match self {
                ConsList::Cons(value, mut next) => {
                    if *value == target {
                        return next.clone();
                    }
                    match next {
                        ConsList::Cons(next_v, nnext) => {
                            if *next_v == target {
                                next = nnext;
                                return self.clone();
                            } else {
                                nnext.remove(target)
                            }
                        }
                        ConsList::Nil => ConsList::Nil,
                    }
                }
                ConsList::Nil => ConsList::Nil,
            }
        }
    }
}
