fn main() {
    println!("Hello, world!");
        /*
     * 复合类型1：tuple
     */
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // // 使用元素的索引号，访问tup中的变量，
    // println!("{}, {}, {}", tup.0, tup.1, tup.2);
    // // 使用模式解构
    // let (x, y, z) = tup;
    // println!("解构赋值：{}, {}, {}", x, y, z);

    /*
     *复合类型2
     */
    // 数组的定义：
    let arr1 = [1, 2, 3, 4, 5];
    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "June", "July", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    // 带类型标注的数组定义
    let a:[i32;5] = [1, 2, 3, 4, 5]; 
    // 特殊数组的定义：所有的元素都相同的情形
    let b = [3;5];
    // println!("{}", b);
    
    // 对数组内容的访问
    let first = months[0];
    let second = months[1];
    println!("first: {}", first);
    println!("second: {}", second);

    // 访问超过数组索引范围
    // 简单索引检查，编译前直接报错
    // let no_value = months[12];
    let idx_arr = [12, 13, 14];
    let no_value = months[idx_arr[1]];
    println!("no_value: {}", no_value);
}
