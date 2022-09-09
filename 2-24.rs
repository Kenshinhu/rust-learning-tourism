use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("zhangsan", 79);
    map.entry("zhangsan").or_insert(97);

    println!("map: {:?}", map);
}
