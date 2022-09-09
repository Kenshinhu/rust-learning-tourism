use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("zhangsan", 99);

    println!("zhangsan: {:?}", map.get("zhangsan"));

    println!("test none: {:?}", map.get("lisi"));
}
