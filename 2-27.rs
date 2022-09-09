use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("zhangsan", 80);

    println!("zhangsan : {} ", map["zhangsan"]);
    // println!("wangwu : {} ", map["wangwu"]);
}
