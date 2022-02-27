/*
Unsafe Rust存在原因：
1. 静态分析是保守的
2. 计算机硬件本身就是不安全的，Rust需要能够进行底层系统的编程
*/

/*
Unsafe "超能力"
1. 解引用原始指针
2. 调用unsafe函数或方法
3. 访问或修改可变的静态变量
4. 实现unsafe trait

需要注意的是unsafe并没有关闭借用检查器或停用其它安全检查
*/

static HELLO_WORLD: &str = "Hello world";
static mut COUNTER: u32 = 0;
fn main() {
    println!("Hello, world!");
    /*
     1. 解引用原始指针
       - 可变的：*mut T
       - 不可变的：*const T。 即指针在解引用后不能直接对其进行赋值

       ◆：原始指针可以忽略借用规则
         - 允许通过同时具有不可变和可变指针或多个指向同一位置的可变指针
         - 无法保证能指向合理的内存
         - 允许为null
         - 不实现任何自动清理
    */
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1: {:?}", *r1);
        println!("r2: {:?}", *r2);
    }
    let address = 0x012345usize;
    // let r = address as *const i32;
    // unsafe {
    // println!("r: {:?}", *r);
    // }

    /*
      2. unsafe函数或方法
         在函数或方法定义前添加unsafe关键字
    */
    unsafe {
        dangerous();
    }

    /*
     3. 访问或修改一个可变静态变量

    */
    println!("name is: {}", HELLO_WORLD);
    add_to_count(3);
    unsafe {
        println!("COUNTE: {}", COUNTER);
    }

    /*
      实现不安全 trait
      当一个trait中存在至少一个方法拥有编译器无法校验的不安全因素时，就称这个trait是不安全的
      声明一个不安全的trait就是在其定义添加 unsafe关键字
    */
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe fn dangerous() {}
