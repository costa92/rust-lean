#[allow(dead_code)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

// 解构嵌套的结构体和枚举
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

// 解构结构体和元组

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }

        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            )
        }
        _ => (),
    }

    // let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    println!("feet: {},inches:{}", feet, inches);
    println!("x: {}, y: {}", x, y);

    // 定长数组
    let arr: [u16; 2] = [114, 514];
    let [x, y] = arr;

    assert_eq!(x, 114);
    assert_eq!(y, 514);

    // 不定长数组

    let arr_one: &[u16] = &[114, 514];

    if let [x, ..] = arr_one {
        assert_eq!(x, &114)
    }

    if let [.., y] = arr_one {
        assert_eq!(y, &514)
    }

    let arr_one: &[u16] = &[];
    assert!(matches!(arr_one, [..]));
    assert!(!matches!(arr_one, [x, ..]));

    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let _x = 5;
    let y = 10;

    let s = Some(String::from("Hello!"));
    // if let Some(_s) = s { // 修改成
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    let origin = Points { x: 0, y: 0, z: 0 };
    match origin {
        Points { x, .. } => println!("x is {}", x),
    }

    // .. 来忽略元组中间的某些值

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
    // match numbers {
    //     (.., second, ..) => {
    //         println!("Some numbers: {}", second);
    //     }
    // }
    //
    //

    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}

struct Points {
    x: i32,
    y: i32,
    z: i32,
}

// 忽略模式中的值
fn foo(_: i32, y: i32) {
    println!("The code only users the y paramenter: {}", y);
}
