fn main() {
    println!("Hello, world!");
}



pub fn add(a: i32,b: i32) -> i32 {
    a + b
}

#[allow(dead_code)]
fn bad_add(a: i32,b: i32) -> i32 {
    a - b
}

// 单元测试
#[cfg(test)]
mod tests{
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
    #[test]
    fn test_bad_add() {
        assert_eq!(bad_add(1, 2), 3);
    }
}

// 运行命令： cargo test
// 运行特定的测试  cargo test test_bad_add

// 测试 panic

pub fn divide_non_zero_result(a: u32,b:u32) -> u32 {
    if b == 0 {
        panic!("divided by zero");
    } else if a < b {
        panic!("Divided result is zero");
    }
    a / b
}

#[cfg(test)]
mod tests2 {
    use super::*;
    #[test]
    fn test_divide() {
        assert_eq!(divide_non_zero_result(10, 2), 5);
    }
    #[test]
    #[should_panic]
    fn test_any_panic() {
        divide_non_zero_result(1, 0);
    }
    #[test]
    #[should_panic(expected = "Divided result is zero")]
    fn test_specific_panic() {
        divide_non_zero_result(1, 10);
    }
}
// 运行命令： cargo test

// 运行特定的测试
// 要运行多个测试，可以仅指定测试名称中的一部分，用它来匹配所有要运行的测试。
// `cargo test panic`  运行所有名称中包含“panic”的测试

// 忽略测试

pub fn add_two(a: i32,b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests3 {
    use super::*;
    #[test]
    fn test_add_two() {
        assert_eq!(add_two(1, 2), 3);
    }

    #[test]
    fn test_add_two_hundred() {
        assert_eq!(add_two(100, 2), 102);
        assert_eq!(add_two(2, 100), 102);
    }

    #[test]
    #[ignore]
    fn test_add_two_three() {
        assert_eq!(add_two(1, 2), 3);
    }
}
// 可以把属性 #[ignore] 赋予测试以排除某些测试，或者使用 cargo test -- --ignored 命令来运行它们。