struct Rectangle1<T> {
    width: T,
    height: T,
}

struct Rectangle2<T, U> {
    width: T,
    height: U,
}

impl<T> Rectangle1<T> {
    fn width(&self) -> &T {
        &self.width
    }

    fn height(&self) -> &T {
        &self.height
    }
}

impl Rectangle1<i32> {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

impl<T, C> Rectangle2<T, C> {
    fn width(&self) -> &T {
        &self.width
    }

    fn height(&self) -> &C {
        &self.height
    }
}

fn main() {
    let rect1 = Rectangle1 {
        width: 8,
        height: 2,
    };

    println!("width:{} height:{}", rect1.width(), rect1.height());
    println!("rect1 area : {}", rect1.area());

    let rect2 = Rectangle2 {
        width: 8,
        height: 2.2,
    };
    println!("width:{} height:{}", rect2.width(), rect2.height());
}
