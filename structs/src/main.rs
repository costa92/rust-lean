use ::std::fmt::Debug;
use std::i32;

fn main() {
    println!("Hello, world!");

    // 实例化结构体
    let user1 = User {
        email: String::from("test@example.com"),
        username: String::from("test"),
        active: true,
        sign_in_count: 1,
    };
    //  有几点值得注意:
    // 1. 结构体中的字段没有指定顺序
    // 2. 结构体中的字段必须全部初始化
    // 访问结构体中的字段
    println!("user1.email: {}", user1.email);
    println!("user1.username: {}", user1.username);
    println!("user1.active: {}", user1.active);
    println!("user1.sign_in_count: {}", user1.sign_in_count);

    // 修改结构体中的字段

    let mut user2 = User {
        email: String::from("test@example.com"),
        username: String::from("test"),
        active: true,
        sign_in_count: 1,
    };
    user2.email = String::from("user2@example.com");
    println!("user2.email: {}", user2.email);

    let user3 = bulid_user(String::from("user3@example.com"), String::from("username"));
    println!("user3.email: {}", user3.email);

    let user4 = bulid_user_v1(
        String::from("user4@example.com"),
        String::from("username_v4"),
    );
    println!("user4.email: {}", user4.email);

    // 结构体更新语法
    let user2 = User {
        email: String::from("refectuser@example.com"), // 保留user2.email的值
        ..user4
    };

    println!("refect user user2.email: {}", user2.email);

    println!("{:?}", user2); // User { active: true, username: "username_v4", email: "refectuser@example.com", sign_in_count: 1 }
    println!("{:#?}", user2);
    // User {
    //     active: true,
    //     username: "username_v4",
    //     email: "refectuser@example.com",
    //     sign_in_count: 1,
    // }
    //

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{:?}", black);
    println!("{:#?}", origin);

    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    println!("f1_name: {}", f1_name);
    println!("{} is f1_length: {}", f1_name, f1_length);

    // let subject = AlwaysEqual;
    // // 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
    // impl SomeTrait for AlwaysEqual {
    //     // add code here
    // }
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

// 元组结构体(Tuple Struct)
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

// 简化结构体创建过程
fn bulid_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn bulid_user_v1(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
// Defining and Instantiating Structs
// 通过关键词 struct 和花括号来定义一个结构体
// 一个清晰明确的结构体 名称
// 几个有名字的结构体 字段
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
