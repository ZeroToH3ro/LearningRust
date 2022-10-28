struct Point <T,U> {
    x: T,
    y: U
}

impl<T,U> Point<T,U> {
    fn mixed<V,W>(self, other: Point<V,W>) -> Point<T,W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

fn main() {
    let p1 = Point {x: 1.5, y: 5};
    let p2 = Point {x: "Minh", y:'c'};

    let p3 = p1.mixed(p2);
    println!("{}, {}", p3.x, p3.y);
}