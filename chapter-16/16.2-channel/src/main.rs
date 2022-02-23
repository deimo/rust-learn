use std::{sync::mpsc, thread, time::Duration};

fn main() {
    // 初次报错，是因为Rust编译器无法准确推测出发送端和接收端的数据
    let (tx, rx) = mpsc::channel();

    // 使用clone方法复制一个新的sender
    let tx1 = mpsc::Sender::clone(&tx);

    // 创建线程1，使用tx1发送者进行发送
    thread::spawn(move || {
        let vals = vec![
            String::from("1: hi"),
            String::from("1: from"),
            String::from("1: the"),
            String::from("1: thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    // 创建线程2：使用默认创建出的tx发送数据
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for received in rx {
        println!("Got. {}", received);
    }

}
