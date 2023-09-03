// struct Struct {
//     e: i32,
// }

// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
//
//     let _z = 5;
//     let _b = 10;
//     println!("Hello, world!");
//
//     let (a, b, c, d, e);
//     (a, b) = (1, 2);
//     // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
//     [c, .., d, _] = [1, 2, 3, 4, 5];
//     Struct { e, .. } = Struct { e: 5 };
//
//     assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
// }
//
//

// fn main() {
//     let x = 5;
//
//     // 在 main 函数的作用域内对之前的x 进行了隐藏
//     //
//     let x = x + 1;
//     {
//         // 在当前的花括号作用域内，对之前的x 进行了隐藏
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {}", x);
//     }
//     println!("The value of x in the outer scope is: {}", x);
//
//     // 字符串类型
//     let mut spaces = "  ";
//     // usize 数值类型
//     let spaces = spaces.len();
// }

// fn main() {
//     let a: u8 = 255;
//     let b = a.wrapping_add(22);
//     println!("{}", b);
//
//     let c = a.checked_add(22);
//     println!("{:?}", c);
//
//     let d = a.overflowing_add(22);
//     println!("{:?}", d);
//     println!("{}", d.0);
//
//     let e = a.saturating_add(22);
//     println!("{}", e);
// }
//
//

// fn main() {
//     let abc: (f32, f32, f32) = (0.1, 0.2, 0.3); // 元组
//     let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3); // 数组
//
//     println!("abc (f32)");
//     println!("  0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
//     println!("  0.3: {:x}", (abc.2).to_bits());
//     println!();
//
//     println!("xyz (f64)");
//     println!("  0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
//     println!(" 0.3: {:x}", (xyz.2).to_bits());
//
//     assert!(abc.0 + abc.1 == abc.2);
//     assert!(xyz.0 + xyz.1 == xyz.2);
// }

// fn main() {
//     let x = (-42.0_f32).sqrt();
//     if x.is_nan() {
//         println!("{} is NaN", x);
//     }
// }
//

// fn main() {
//     // 加法
//     let sum = 5 + 10;
//     println!("sum: {}", sum);
//
//     // 减法
//     let difference = 95.5 - 4.3;
//     println!("difference: {}", difference);
//
//     // 乘法
//     let product = 4 * 30;
//     println!("product: {}", product);
//
//     // 除法
//     let quotient = 56.7 / 32.2;
//     println!("quotient: {}", quotient);
//     // 取余
//     let remainder = 43 % 5;
//     println!("remainder: {}", remainder);
// }
//
// fn main() {
//     // 编译器会进行自动推导，给予twenty i32的类型
//     let twenty = 20;
//     // 类型标注
//     let twenty_one: i32 = 21;
//     // 通过类型后缀的方式进行标注: 22 是i32 类型
//     let twenty_two = 22i32;
//     // 只有同样类型,才能运算
//     let addition = twenty + twenty_one + twenty_two;
//     println!(
//         "{} + {} + {} = {}",
//         twenty, twenty_one, twenty_one, addition
//     );
//
//     // 对于较长的数字，可以用_进行分割,提升可读性
//     let one_million: i64 = 1_000_000;
//     println!("{}", one_million.pow(2));
//
//     // 定义一个 f32 数组，其中 42.0 是 f32 类型
//     let forty_twos = [42.0, 42f32, 42.0_f32];
//
//     // 打印数组中第一个元素
//     println!("{:.2}", forty_twos[0]);
// }

fn main() {
    // 二进制为 00000010
    let a: i32 = 2;
    // 二进制为 00000011
    let b: i32 = 3;

    println!("(a & b) value is {} ", a & b);
    println!("(a | b) value is {} ", a | b);
    println!("(a ^ b) value is {} ", a ^ b);
    println!("(!a) value is {} ", !a);
    println!("(a << b) value is {} ", a << b);
    println!("(a >> b) value is {} ", a >> b);

    let mut a = a;
    // 注意这些计算符除了！之外都可以加等号进行赋值 （因为 != 要来判断不等于）
    //
    a <<= b;
    println!("a <<= b is {}", a);

    for i in 1..=10 {
        println!("{}", i);
    }

    for i in 'a'..='z' {
        println!("{}", i);
    }
}
