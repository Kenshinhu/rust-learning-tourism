fn main() {
    let mut p: Vec<i32> = Vec::new();
    p.push(1);
    p.push(2);
    p.push(3);

    p[1] = 5;

    println!("{:?}", p);
}
