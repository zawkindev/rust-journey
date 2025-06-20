fn main() {
    struct UnitLike; //void

    struct Color(u8, u8, u8);
    let black = Color(0, 0, 0);

    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn new(width: u32, height: u32) -> Rectangle {
            Rectangle { width, height }
        }

        fn area(self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }

        fn increase_size(&mut self, width: u32, height: u32) {
            self.width += width;
            self.height += height;
        }

        fn set_width(mut self, width: u32) -> Self {
            self.width = width;
            self
        }
    }

    let rec1 = Rectangle {
        width: 30,
        height: 30,
    };

    let rec2 = Rectangle::new(30, 50);

    let rec3 = Rectangle { width: 30, ..rec1 };

    println!("The area of the rec1: {}", rec1.area());

    pub struct Person {
        pub name: String,
        age: u8,
    }

    impl Rectangle {
        fn print(&self) {
            println!("Rectangle: {} x {}", self.width, self.height);
        }
    }

    #[derive(Default)]
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point::default();

    let other_point = Point {
        x: 10,
        ..Point::default()
    };

    impl Default for Rectangle {
        fn default() -> Rectangle {
            Rectangle {
                width: 10,
                height: 10,
            }
        }
    }
}
