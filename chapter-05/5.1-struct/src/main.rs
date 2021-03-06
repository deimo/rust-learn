// 声明并定义一个struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 结构体作为函数的返回值
// fn build_user(email: String, username: String) -> User {
//     User {
//         email: email,
//         username: username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// 结构体初始化时的字段与变量同名的简写形式
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    println!("Hello, world!");
    // ## 实例化一个struct，需注意以下两点
    /*
     * 1. 实例化的字段顺序与声明定义时的字段顺序无关
     * 2. 实例化时所有的字段都必须赋值，负责报错
     */
    let mut user1 = User {
        email: String::from("acb@126.com"),
        username: String::from("Nikky"),
        active: true,
        sign_in_count: 556,
    };

    // ## 访问结构体中的字段:使用点[.]标记法
    user1.email = String::from("anotheremail@example.com");
    /*
     *特别地:
     * 1. 一旦struct的实例被声明为可变的，那么其实例中所有的字段都是可变的
     * 2. 当字段名与字段值对应的变量名相同时，可以省略字段名，直接使用变量名用以简写
     */

    // ## struct更新语法: 基于某个struct实例来创建一个新实例，核心是变量解构
    let user2 = User {
        email: String::from("another@example"),
        username: String::from("anotherusername567"),
        ..user1
    };

    // ## Tuple struct
    /*
     * 定义类似tuple的struct，叫做tuple struct
     * 其表现特点：
     * Tuple struct整体有个名，但里面的元素没有名
     * 使用场景：
     * 想给一个tuple起名，并为其添加自定义类型，而又无需关心tuple内每个元素的字段名
     */
    let red = Color(1, 0, 0);
    let origin = Point(0, 0, 0);
    // 使用模式匹配解构tuple struct的值
    let Color(r, g, b) = red;
    println!("r: {}", r);
    // 使用基于角标
    println!("red.0: {}", red.0)

    // ## 关于struct数据的所有权规则
    //   struct User {
    //     username: String,
    //     email: String,
    //     sign_in_count: u64,
    //     active: bool,
    //   }
    // 上述struct中的字段使用了String，而不是&str
    /*
     * 1. 该struct实例拥有其所有的数据
     * 2. 只要struct实例是有效的，那么里面的字段数据也是有效的
     * 3. struct实例也可以存放引用，但这需要使用到生命周期
     *    ★ 生命周期可保证只要struct中的实例是有效的，那么其字段中的引用值也会是有效的
     */
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
