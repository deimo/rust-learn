use std::rc::Rc;

#[cfg(test)]
#[test]
fn test1() {
    // let s = String::from("hello world");
    // let a = Box::new(s);
    // let b = Box::new(s);
}

#[test]
fn test2() {
    let a = Rc::new(String::from("hello, world"));
    let b = Rc::clone(&a);
    assert_eq!(2, Rc::strong_count(&a));
}

#[test]
fn test3() {
    let a = Rc::new(String::from("test ref counting"));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Rc::clone(&a);
    println!("count after creating b = {}", Rc::strong_count(&b));

    {
        let c = Rc::clone(&a);
        println!("count after creating c = {}", Rc::strong_count(&c));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}


struct Owner {
    name: String
}

struct Gadget {
    id: i32,
    owner: Rc<Owner>
}

#[test]
fn test5() {
    let gadget_owner = Rc::new(Owner {
        name: "Gadget Man".to_string()
    });
    let gadget1 = Gadget {
        id: 1,
        owner: Rc::clone(&gadget_owner)
    };
    let gadget2 = Gadget {
        id: 2,
        owner: Rc::clone(&gadget_owner)
    };
    // println!("count = {}", Rc::strong_count(&gadget_owner));
    drop(gadget_owner);
    println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
    println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);
}