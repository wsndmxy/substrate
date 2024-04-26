
trait CalculableBound {
    fn area(&self) -> f64;
}
struct Circle {
    radius: f64,
}

impl CalculableBound for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius 
    }
}
struct Triangle {
    base: f64,
    height: f64,
}
impl CalculableBound for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height 
    }
}
struct Square {
    side: f64,
}
impl CalculableBound for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

struct Rectangle {
    width1: f64,
    width2: f64,
}

impl CalculableBound for Rectangle {
    fn area(&self) -> f64 {
        self.width1 * self.width2
    }
}

fn print_area<T: CalculableBound>(shape: T) {
    println!("Area: {}", shape.area());
}
fn main() {
    let circle = Circle { radius: 3.0};
    let triangle = Triangle { base: 5.5, height: 6.6 };
    let rectangle = Rectangle { width1: 4.0, width2: 8.0 };
    let square = Square { side: 108.0 };

    print_area(circle);
    print_area(triangle);
    print_area(rectangle);
    print_area(square);
}