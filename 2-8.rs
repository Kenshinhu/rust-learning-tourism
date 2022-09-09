#[derive(Debug)]
enum ColorNoParam {
    Red,
    Yellow,
    Blue,
}

fn main() {
    let color_no_param = ColorNoParam::Yellow;

    match color_no_param {
        ColorNoParam::Red => println!(" {:?} ", ColorNoParam::Red),
        ColorNoParam::Yellow => println!(" {:?} ", ColorNoParam::Yellow),
        ColorNoParam::Blue => println!(" {:?} ", ColorNoParam::Blue),
    }
}
