use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&str, i32> = HashMap::new();

    map.insert("zhangsan", 78);
    map.insert("lisi", 70);
    map.insert("wangwu", 89);

    let result = map.remove("wangwu");
    print!("After remove: {:?}", result);

    println!("origin: {:?}", map)
}
