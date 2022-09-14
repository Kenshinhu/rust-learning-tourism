use std::fmt::{Display, Formatter, Result};

trait Geometry {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
}

struct Rectangle {
    width: f32,
    height: f32,
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Rectangle : ( {}, {} )", self.width, self.height)
    }
}

impl Geometry for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
    fn perimeter(&self) -> f32 {
        (self.width + self.height) * 2.0
    }
}

fn print(geometry: impl Geometry + Display) {
    println!(
        "{}, area:{}, perimeter:{}",
        geometry.fmt,
        geometry.area(),
        geometry.perimeter()
    );
}

fn main() {
    let rect = Rectangle {
        width: 3.0,
        height: 5.9,
    };

    print(rect);
}
