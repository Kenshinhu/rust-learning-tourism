fn main() {
    let mut s = String::from("Hello World");
    s.insert(5, ',');
    s.insert_str(6, "\"d\"");

    println!("{}", s);
}
