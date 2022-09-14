trait Geometry {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
}

struct Rectangle {
    width: f32,
    height: f32,
}

impl Geometry for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn perimeter(&self) -> f32 {
        (self.width + self.height) * 2.0
    }
}

fn print(geometry: impl Geometry) {
    println!(
        "area: {}, perimeter: {}",
        geometry.area(),
        geometry.perimeter()
    );
}

fn main() {
    let rect = Rectangle {
        width: 8.0,
        height: 4.0,
    };

    print(rect);
}
