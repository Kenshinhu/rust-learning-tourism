fn main() {
    let mut s = String::from("Hello");
    s.push(',');
    s.push_str(" World!");

    println!("{}", s);
}
