use std::fmt::Display;

struct Parity<T> {
    x: T,
    y: T,
}

impl<T> Parity<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Parity<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
fn main() {
    println!("Hello, world!");
    let parity = Parity::new(1, 2);
    parity.cmp_display();
}
