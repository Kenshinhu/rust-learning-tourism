fn main() {
    let s = String::from("I’m 中山人");

    println!("{}: len = {}", s, s.len());

    let s = String::from("’");
    println!("s = {:?},  len = {}", s, s.len());
}
