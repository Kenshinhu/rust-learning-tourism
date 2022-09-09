use std::collections::HashMap;

fn main() {
    let mut hashmap: HashMap<&str, i32> = HashMap::new();

    hashmap.insert("zhangsan", 79);
    hashmap.insert("lisi", 90);
    hashmap.insert("wangwu", 84);

    println!("Before {:?}", hashmap);

    for (_, scores) in hashmap.iter_mut() {
        *scores += 2;
    }

    println!("After {:?}", hashmap);
}
