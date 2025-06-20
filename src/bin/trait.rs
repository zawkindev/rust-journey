fn main() {
    trait Shape {
        fn area(&self) -> f64;
    }

    struct Circle {
        radius: f64,
    }

    impl Circle {
        fn new(radius: f64) -> Circle {
            Circle { radius }
        }
    }

    impl Shape for Circle {
        fn area(&self) -> f64 {
            3.4 * self.radius * self.radius
        }
    }

    struct Rectangle {
        width: f64,
        height: f64,
    }

    impl Rectangle {
        fn new(width: f64, height: f64) -> Rectangle {
            Rectangle { width, height }
        }
    }

    impl Shape for Rectangle {
        fn area(&self) -> f64 {
            self.width * self.height
        }
    }

    let circle = Circle::new(5.0);
    let rectangle = Rectangle::new(4.0, 3.0);

    fn print_area(shape: &dyn Shape) {
        println!("Area: {}", shape.area());
    }

    print_area(&circle);
    print_area(&rectangle);
}
