use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main(){
    // let f = File::open("hello.txt");
    // 使用matc表达式处理Result
    // let f = match f {
    //     Ok(file) => file,
    //     Err(err) => {
    //         panic!("Error opening file {:?}", err);
    //     }
    // };
    // ---------------------------------------------------------------

    // 使用match表达式匹配不同的错误：可以看到match很有用，但是很原始，需要嵌套很多match
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Error creating file: {:?}", e),
    //         },
    //         other_error => panic!("Error opening the file: {:?}", other_error),
    //     }
    // };

    // ---------------------------------------------------------------
    // 使用闭包简化match表达式
    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Error creating file {:?}", error);
    //         })
    //     } else {
    //         panic!("Error opening file: {:?}", error);
    //     }
    // });

    // ---------------------------------------------------------------
    
    /*
     * Result<T, E>枚举的常见方法
     * - unwrap() :match表达式的一个快捷方法
     *   - 如果Result的结果是Ok，返回Ok变体里关联的值
     *   - 如果Result的结果是Err，则调用panic!宏,但是无法自定义错误信息
     * 
     * - expect(): 和unwrap类似，但是可以指定panic!宏附带的错误信息
     */
    // let f= File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("无法打开文件");
    
    /*
     * 传播错误:将错误返回给调用者
     * - [?] 运算符：传播错误的一种快捷方式
     *      > 如果表达式结果的Result是Ok，Ok中的值就是表达式的结果，
     *        如果表达式结果的Result是Err，Err就是整个函数的返回值，相当于使用了return
     *      > [?] 运算符只能用于返回类型为Result枚举的函数
     *      > [?] 运算符实际上是隐式调用了 from 函数（操作的错误类型实现了转换为所返回的错误类型的from函数）
     */
     let result = read_username();
     match result {
         Ok(username) => println!("username is {}", username),
         Err(_) => panic!("io error occured")
     }
    
}

fn read_username()-> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}



fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

fn read_username_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}


fn read_username_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}