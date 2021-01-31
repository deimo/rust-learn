fn main() {
    /*
     * 1. 创建字符串对象String
     * - String::new();
     * - to_string();
     * - String::from();
     */
    // let mut s = String::new();
    // let data = "initial_contents";
    // let s = data.to_string();
    // let s1 = "initial contens".to_string();
    // let s = String::from("initial contents");

    /*
     * 2. 更新String
     * - push_str(): 把一个字符串切片附加到String
     * - push(): 把单个字符附加到String
     * - '+' 运算符：连接字符串
     *    > 类似于使用了fn add(self, s: &str) -> String{}
     *    > 获取左操作数的所有权，而不会获取右操作数的所有权
     *    > 实现细节上使用了所谓的 ”解引用强制转换“ (deref coercion)
     * - format!宏：用于连接多个字符串
     */
    // let mut s = String::from("foo");
    // let s1 = String::from("bar");
    // s.push_str(&s1);
    // println!("{}", s);
    // println!("{}", s1);

    // let mut s = String::from("lo");
    // s.push('l');

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("World!");
    // let s3 = s1 + &s2;
    // println!("{}", s3);
    // println!("{}", s1);  // s1 的所有权在拼接的时候已经移动了
    // println!("{}", s2);
    
    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    // let s3 = s1 + "-" + &s2 + "-" + &s3;
    // println!("{}", s3);
    // let s = format!("{}-{}-{}", s1, s2, s3); // 不获取任何参数的所有权
    // println!("s is {}", s);
    // println!("s1 is {}", s1);
    // println!("s2 is {}", s2);
    // println!("s3 is {}", s3);

    /*
     * 3. Rust不允许使用index语法访问字符串中的元素
     */
    let s1 = String::from("hello");
    let len = s1.len();
    println!("{}", len);

    /*
     * 4. 切割String []
     * - 必须谨慎使用
     * - 如果切割时跨越了字符边界，程序就会panic
     */


     /*
      * 5.字符串遍历
      * - 对于标量值：chars() 
      * - 对于字节值：bytes() 
      */


}
