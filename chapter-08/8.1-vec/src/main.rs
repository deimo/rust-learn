enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {
    /*
     * 1. 创建 Vector
     */
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];

    /*
     * 2. 更新 Vector
     */
    // let mut v = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);
    // v.push(4);

    /*
     * 3. 删除 Vector
     * - 与任何其它struct一样，当Vector离开作用域后会被清理，所有元素也会被清理
     */
    // let v = vec![1, 23, 4];

    /*
     * 4. 读取Vector中的元素
     * 两种方式：两种方式在处理index越界的表现有差别
     *  - 索引
     *  - get 方法
     */
    // let v = vec![1, 2, 3, 4, 5];
    // let third = v[200];
    // println!("the third is {}", third);
    // match v.get(200) {
    //     Some(third) => println!("The third elemetn is {}", third),
    //     None => println!("there is no third element"),
    // }

    /*
     * 5. Vector的所有权和借用规则
     * - 不可在同一作用域内同时拥有对某个值的可变和不可变引用
     */
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];  //  获取不可变引用
    // v.push(6);
    // println!("the first elemnt is {}", first);

    /*
     * 6. 遍历Vector中的值
     */
    // let mut v = vec![100, 32, 57];
    // for i in &v {
    //     println!("{}", i);
    // }
    // for i in &mut v {
    //     // println!("{}", i);
    //     *i += 50;
    // }
    // println!(">>>>>>>>>>>>>>>>");
    // for i in v {
    //     println!("{}", i);
    // }


    // 使用Vector存储有限种可能的enum类型
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(19.12),
    ];
}
