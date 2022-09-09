fn main() {
    let s = String::from("aaabbccddeeff");
    let s1 = s.replace("aa", "1");
    let s2 = s1.replacen("b", "2", 2);

    println!("{}", s);
    println!("{}", s1);
    println!("{}", s2);
}
