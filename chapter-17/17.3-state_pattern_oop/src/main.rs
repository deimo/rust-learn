// 面向对象的状态模式：
// - 一个值拥有的内部状态由数个状态独享表达而成，而值的行为则随着内部状态的改变而改变

use state_pattern::Post;


fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());


    post.request_review();
    assert_eq!("", post.content());


    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
    println!("Hello, world!");
}
