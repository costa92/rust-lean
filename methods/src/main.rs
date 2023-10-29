#[derive(Debug)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    // new 是 Circle 的关联函数，因为它的第一个参数不是 self，且New 并不是关键字
    // 这种方法往往用于初始化当前结构体的一些数据
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }

    // Circle 的方法， &self 表示借用当前的Circle 结构体
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
#[allow(unused)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 area: {}", rect1.area());

    let circle = Circle::new(0.0, 0.0, 2.0);

    println!("circle area: {}", circle.area());

    let m = Message::Write(String::from("hello"));
    m.call();
}
