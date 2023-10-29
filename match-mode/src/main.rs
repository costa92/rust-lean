#[allow(dead_code)]
enum Direction {
    East,
    West,
    North,
    South,
}
#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
#[allow(dead_code)]
enum IpAddr {
    Ipv4,
    Ipv6,
}

fn main() {
    let direction = Direction::South;
    match direction {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("North or South")
        }
        _ => println!("West"),
    };

    let coin_val = value_in_case(Coin::Nickel);
    println!("Coin value: {}", coin_val);

    // 因为这里匹配到 _ 分支，所以将 "::1" 赋值给了 ip_str。
    let ip1 = IpAddr::Ipv4;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };

    println!("IP: {}", ip_str);

    let cents_val = value_in_cents(UsCoin::Quarter(UsState::Alaska));

    println!("Cents value: {}", cents_val);
}

fn value_in_case(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// 模式绑定
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum UsCoin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: UsCoin) -> u8 {
    match coin {
        UsCoin::Penny => 1,
        UsCoin::Nickel => 5,
        UsCoin::Dime => 10,
        UsCoin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
