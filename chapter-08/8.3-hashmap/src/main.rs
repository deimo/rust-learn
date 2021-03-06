use std::collections::HashMap;

fn main() {
    // println!("Hello, world!");
    /*
     * 1. 创建HashMap;
     * - HashMap::new();
     * - 从Vector到HashMap：使用collect()
     *   > Vector的元素类型为Tuple
     *   > Tuple有两个值：一个作为K，一个作为V
     */
    // let mut scores = HashMap::new();
    // 使用collect
    // let teams = vec![String::from("blue"), String::from("Yellow")];
    // let inital_scores = vec![10, 50];
    // let scores: HashMap<_, _> = teams.iter().zip(inital_scores.iter()).collect();

    /*
     * 2. 更新HashMap;
     * - 插入值 self.insert();
     *
     */
    // scores.insert(String::from("Blue"), 10);
    /*
     * 3. HashMap和所有权
     * - 对于实现了Copy trait的类型，值会被复制到HashMap中
     * - 对于拥有所有权的值（如String），值会被移动，所有权会转移给HashMap
     *
     */

    // let field_name = String::from("Favorite color");
    // let field_value = String::from("blue");

    // let mut map = HashMap::new();
    //  map.insert(field_name, field_value);
    // map.insert(&field_name, &field_value);
    // println!("{}: {}", field_name, field_value);

    /*
     * 4. 访问HashMap 中的值
     * - get(), 返回Option<&V>
     */
    // let mut scores = HashMap::new();
    // scores.insert(String::from("blue"), 10);
    // scores.insert(String::from("Yello"), 50);
    // let team_name = String::from("blue");
    // let score = scores.get(&team_name);
    // let mut result:i32 = -1 ;
    // match score {
    //     Some(s) => {
    //         println!("{}", s);
    //         result = *s;
    //     },
    //     None => println!("team not exist"),
    // }
    // println!("resut is {}", result);

    /*
     * 5. 遍历HashMap
     */
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    // for (k, v) in &scores {
    //     println!("{}: {}", k, v);
    // }
    /*
     * 6. 更新HashMap
     * entry(): 检查指定的K是否对应一个V
     * 
     * Entry的or_insert()方法：
     * - 返回：
     *   > 如果K存在，返回到对应的V的一个可变引用
     *   > 如果K不存在，将方法参数作为K的新值插入进去，返回到这个值得可变引用
     * - K已存在
     *   > 替换现有V：直接insert
     *   > 保留旧的，忽略新的
     *   > 合并现有的V和新的V
     * - K不存在
     *   > 添加一堆K，V
     */
    // let mut scores = HashMap::new();
    // scores.insert(String::from("blue"), 10);
    // scores.insert(String::from("blue"), 50);
    // println!("{:?}", scores);
    
    // let mut scores = HashMap::new();
    // scores.insert(String::from("blue"), 10);
    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("blue")).or_insert(50);
    // println!("{:?}", scores);
    
    /*
     *  一个用于统计一段文本中各个单词出现的次数例子
     */
     let text = "hello world wonderful world";
     let mut map = HashMap::new();

     for word in text.split_whitespace() {
         let count = map.entry(word).or_insert(0);
         *count += 1;
     }
     println!("{:#?}", map)
}
