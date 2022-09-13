fn main() {
    let v = [1, 2, 3];
    let result: i32 = v.iter().map(|x| x + 3).sum();

    println!("result:{}", result);
}
