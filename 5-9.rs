use std::fmt::{Display, Formatter, Result};

trait Geometry {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
}

struct Circle {
    raduis: f32,
}

impl Geometry for Circle {
    fn area(&self) -> f32 {
        3.14 * self.raduis * self.raduis
    }

    fn perimeter(&self) -> f32 {
        3.14 * 2.0 * self.raduis
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Circle : ({})", self.raduis)
    }
}

fn print<T: Geometry + Display>(geometry: T) {
    println!("{}, area:{}", geometry, geometry.area());
}

fn main() {
    let circle = Circle { raduis: 5.9 };
    print(circle)
}
