
// 定义 add 声明宏

macro_rules! add {
    // 第 一个 分支，匹配两个参数
    ($a:expr, $b: expr) => {
        $a + $b
    };

    // 第二个分支，当只有一个参数时，返回参数本身
    ($a:expr) => {
        $a
    };
    // 第三个分支，匹配多个参数
    // ($a:expr, $b: expr, $($t: expr), +) => {
    //     $a + $b + add!($($t), +)
    // };
    ($($a:expr),*) => {
        {
            // 开头要有个0，处理没有任何参数传入的情况
            0
            // 使用 $() 包裹起来，表示匹配 0 个或多个
            $(+ $a)*
        }
    }
}

fn main() {
    let x=1;
    let sum_one = add!(x);  // 调用宏
    let sum_two = add!(x, 2);
    let sum_three = add!(x, 2, 3, 4);

    println!("sum_one: {}", sum_one);
    println!("sum_two: {}", sum_two);
    println!("sum_three: {}", sum_three);

    foo();

}

mod inner {
    super::m!();
    crate::m!();
}

mod toexport {
    #[macro_export] // 注意这里 把m!() 导出 当前 crate 的所有模块都可以使用
    macro_rules! m {
        () => {
          
        };
    }
}

fn foo(){
    self::m!(); // 注意这里 不能使用 super::m!(); 因为 super 只能用在模块内部
    m!(); // 注意这里 不能使用 crate::m!(); 因为 crate 只能用在模块内部
}