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

struct Circle {
    raduis: f32,
}

impl Geometry for Circle {
    fn area(&self) -> f32 {
        3.14 * self.raduis * self.raduis
    }
    fn perimeter(&self) -> f32 {
        3.14 * self.raduis * 2.0
    }
}

fn area_add(geo1: impl Geometry, geo2: impl Geometry) {
    println!(
        "rect.area: {}, circle.area: {}, total area: {}",
        geo1.area(),
        geo2.area(),
        geo1.area() + geo2.area()
    );
}

fn main() {
    let circle = Circle { raduis: 3.0 };
    let rectangle = Rectangle {
        width: 5.0,
        height: 3.0,
    };

    area_add(rectangle, circle);
}
