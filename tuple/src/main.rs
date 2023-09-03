fn main() {
    println!("Hello, world!");
    crate_one();

    match_tup();

    access_tup();

    use_tup();
}
// Path: src/main.rs
// 创建一个的元组
fn crate_one() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup: {:?}", tup);
}

// 用模式匹配解构元组

fn match_tup() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
    println!("The value of z is: {}", z);
}

// 访问元组中的单个元素
// 访问：en：access
fn access_tup() {
    // 用 . 来访问元组中的值
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0; // 500
    let six_point_four = x.1; // 6.4
    let one = x.2; // 1
                   //
    println!("five_hundred: {}", five_hundred);
    println!("six_point_four: {}", six_point_four);
    println!("one: {}", one);
}

// 元组的使用示例

fn use_tup() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("the length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
