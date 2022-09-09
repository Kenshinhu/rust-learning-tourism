#[derive(Debug)]
enum ColorParam {
    Red(String),
    Yellow(String),
    Blue(String),
}

fn main() {
    println!("{:?}", ColorParam::Blue(String::from("blue")));
}
