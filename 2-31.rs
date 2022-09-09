fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from(",  ");
    let s3 = String::from("Rust ");
    let s4 = "World";

    let mut s = s1 + &s2 + s3.as_str() + s4;
    println!("{}", s);
}
