enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

#[derive(Debug)]
enum Direction {
    East,
    West,
    North,
    South,
}

fn test_match() {
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];

    for action in actions.iter() {
        match action {
            Action::Say(msg) => println!("say {}", msg),
            Action::MoveTo(x, y) => println!("move to {}, {}", x, y),
            Action::ChangeColorRGB(r, g, b) => println!("change color to {}, {}, {}", r, g, b),
        }
    }
}

fn test_match_demo() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("North or South");
        }
        _ => println!("West"),
    };
}

fn test_semo_u8_value() {
    let semo_u8_value = 0u8;
    match semo_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

fn test_one_single() {
    let dire = Direction::North;
    match dire {
        Direction::East => print!("East"),
        other => println!("other direction: {:?}", other),
    }
}

fn test_match_if_let() {
    let v = Some(3u8);
    // 方法1
    match v {
        Some(3) => println!("three"),
        _ => (),
    }

    // 方法二 使用 if let 替换
    if let Some(3) = v {
        println!("three");
    }
}

fn main() {
    // test_match();
    // test_match_demo();
    // test_semo_u8_value();
    // test_one_single();

    test_match_if_let();
}
