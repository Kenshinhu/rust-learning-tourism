fn main() {
    let mut s = String::from("low 中文字");
    println!("{:?}", s);

    print!("{:?}", s.pop());
    println!("s = {:?}", s);

    print!("{:?}", s.remove(4));
    println!("s = {:?}", s);

    print!("{:?}", s.truncate(2));
    println!("s = {:?}", s);

    print!("{:?}", s.clear());
    println!("s = {:?}", s);
}
