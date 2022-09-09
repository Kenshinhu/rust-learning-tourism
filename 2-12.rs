fn main() {
    let mut v: Vec<i32> = Vec::with_capacity(10);
    v.push(1);
    v.push(2);
    v.push(3);

    println!("pop : {:?}", v.pop());
    println!("values: {:?}", v);
}
