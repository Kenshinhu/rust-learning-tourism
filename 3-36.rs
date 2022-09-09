fn main() {
    let s = String::from("i'm 中山人");

    for c in s.bytes() {
        print!("{} | ", c);
    }
    println!();
    for c in s.chars() {
        print!("{} | ", c);
    }
}
