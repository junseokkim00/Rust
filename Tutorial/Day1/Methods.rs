struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }

    fn pick_one<T>(&self, a: T, b: T) -> T { // Function Overloading is not allowed in Rust!!!
        if std::process::id() % 2 == 0 {a} else {b}
    }
}

fn main() {
    let mut rect = Rectangle { width: 10, height: 5};
    println!("old area: {}", rect.area());
    rect.inc_width(5);
    println!("new area: {}", rect.area());
    println!("{}",rect.pick_one("apple", "banana"));
    println!("{}",rect.pick_one(1, 10));
}