fn main() {
    let s1 = String::from("Hello");
    let s2 = ", ";
    let s3 = String::from("Ruest");
    let fomatter = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", fomatter);
}
