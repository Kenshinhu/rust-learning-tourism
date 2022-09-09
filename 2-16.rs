fn main() {
    let v = vec![1, 2, 3];
    for i in v {
        print!("{},", i);
    }

    println!("");

    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }
}
