use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    pub fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            List::Cons(_, item) => Some(item),
            List::Nil => None,
        }
    }
}

#[derive(Debug)]
pub struct WhatAboutThis<'a> {
    pub name: String,
    pub nickname: Option<&'a str>,
}


impl<'a> WhatAboutThis<'a> {
    pub fn tie_the_knot(&'a mut self) {
        self.nickname = Some(&self.name[..4]);
    }
}