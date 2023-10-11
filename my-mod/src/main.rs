// 定义名为 my_mod 的模块
mod my_mod {
    // 模块中的选项默认具有私有可见性
    // 在模块中定义一个函数
    pub fn add_two(a: i32,b: i32) -> i32 {
        a + b
    }
}

fn main() {
    println!("Hello, world!");
}
