fn main() {
    println!("Hello, world!");
    let s = get_string();
    println!("{}", s);
    let str = join_str();
    println!("{}", str);
    // 交换
    copy_test();

    str_copy_v1();

    str_copy_v2();

    depth_clone();

    shallow_clone();

    let s1 = String::from("hello");
    takes_ownership(s1);

    let x = 5;
    makes_copy(x);

    let s2 = give_ownership(); // give_ownership 将返回
    println!("{}", s2);
    let s3 = String::from("hello"); // 移给 s3
    let s4 = take_and_give_back(s3); // s3 被移动到 take_and_give_back 中,
    println!("{}", s4);

    zhi();

    calculate();

    mut_change();

    solu_mutl_str();
}

fn get_string() -> String {
    String::from("hello")
}

#[allow(dead_code)]
fn join_str() -> String {
    let mut s = String::from("hello");
    s.push_str(", world!");
    s
}

fn copy_test() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}
// 如果打印 s1 会报错
fn str_copy_v1() {
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1);
    println!("{}, world!", s2);
}

// 这样复制没有错误
fn str_copy_v2() {
    let x: &str = "hello";
    let y = x;
    println!("x = {}, y = {}", x, y);
}

// 深度克隆
fn depth_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

// 浅克隆
// 浅拷贝只发生在栈上，因此性能很高，在日常编程中，浅拷贝无处不在。
fn shallow_clone() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 在这里离开作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

fn give_ownership() -> String {
    // give_ownership 将返回值移到给调用它的函数
    let some_string = String::from("hello"); // some_string 进入作用域
    some_string // 返回 some_string 并移出给调用的函数
}

// take_and_give_back 将传入字符串并返回该值
fn take_and_give_back(a_string: String) -> String {
    // a_string 进入作用域
    a_string // 返回 a_string 并移出给调用的函数
}

// 引用与解引用
// 常规引用是一个指针类型，指向了对象存储的内存地址。在下面代码中，我们创建一个 i32 值的引用 y，然后使用解引用运算符来解出 y 所使用的值:
fn zhi() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// 不可变引用
fn calculate() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 传入所有权
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // s 是对 String 的引用
    s.len()
} // 这是, s 离开作用域，但因为它并不拥有引用值的所有权，所以什么也不会发生
  //
  //

// 修改引用的值
#[allow(dead_code)]
fn change() {
    let s = String::from("hello");
    change_str(&s);
}
// 会报错，因为 s 是不可变的
fn change_str(some_string: &String) {
    // some_string.push_str(", world");
}

// 可变引用
fn mut_change() {
    let mut s = String::from("hello");
    mut_change_str(&mut s);
}

// 可变引用
fn mut_change_str(some_string: &mut String) {
    some_string.push_str(", world");
}

// 可变引用同时只能存在一个
// 不过可变引用并不是随心所欲、想用就用的，它有一个很大的限制： 同一作用域，特定数据只能有一个可变引用：
#[allow(dead_code)]
fn mutl_str() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s; // 报错
    // println!("{}, {}", r1, r2);
}

//解决方法是创建一个新作用域
fn solu_mutl_str() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用
    let r2 = &mut s;
    println!("{}", r2);
}

// 可变引用与不可变引用不能同时存在
fn change_not_change() {
    let mut s = String::from("hello");
    let r1 = &s; // 不可变引用
    let r2 = &s; // 不可变引用
                 // let r3 = &mut s; // 大问题
                 // println!("{}, {}, and {}", r1, r2, r3);
    println!("{}, {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);
} // 老编译器中，r1、r2、r3作用域在这里结束
  // 新编译器中 r3作用域在这里结束
  //

fn dangle() -> &String {
    // dangle 返回一个字符串的引用
    let string = String::from("hello"); // string 是一个新字符串
    &string // 返回字符串 string 的引用
} // 这个 string 什么时候被释放？
  // 会报错，因为 string 在函数结束后就被释放了
  //
  // 解决方法 dangle
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
