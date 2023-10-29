// 简单解释就是：一个变量要么有值：Some(T), 要么为空：None。

// 匹配 Option<T>

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

struct Point {
    x: i32,
    y: i32,
}
#[warn(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six = {:?}; none = {:?}", six, none);

    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("anything"),
    }

    let x = Some(5);

    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // 这里的y是match的y，而不是外部的y
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y); // 这里的y是外部的y，而不是match的y

    let y = 1;
    match y {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let z = 5;
    match z {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let zx = 'c';
    match zx {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p; // 解构
    assert_eq!(0, a);
    assert_eq!(7, b);

    println!("a = {}, b = {}", a, b);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }

        Message::Write(text) => println!("Text message: {}", text),

        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
}
