

struct HasDrop1;
struct HasDrop2;

impl Drop for HasDrop1 {
    fn drop(&mut self) {
        println!("Dropping HasDrop1!");
    }
}

impl Drop for HasDrop2 {
    fn drop(&mut self) {
        println!("Drrpping HasDrop2!");
    }
}

struct HasTwoDrops {
    one: HasDrop1,
    two: HasDrop2,
}

impl Drop for HasTwoDrops {
    fn drop(&mut self) {
        println!("Droppig HasTwoDrops");
    }
}

#[derive(Debug)]
struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo!");
    }
}



#[test]
fn test1() {
    let _x = HasTwoDrops {
        two: HasDrop2,
        one: HasDrop1,
    };
    let _foo = Foo;
    // _foo.drop();
    drop(_foo);
    
    println!("Running!");
}