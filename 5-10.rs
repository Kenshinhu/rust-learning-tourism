use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
fn main() {
    let point = Point { x: 10, y: 9 };
    println!("{}", point);
    println!("{:?}", point);
    println!("{:#?}", point);
}
