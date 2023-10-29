fn main() {
    // test_if();
    // test_for_demo1();
    // test_for_demo2();
    // test_for_demo3();
    // test_for_demo4();
    // test_for_demo5();
    // test_while();
    test_loop();
}
// if 处理
fn test_if() {
    let n = 6;
    if n % 4 == 0 {
        println!("n is divisible by 4");
    } else if n % 3 == 0 {
        println!("n is divisible by 3");
    } else if n % 2 == 0 {
        println!("n is divisible by 2");
    } else {
        println!("n is not divisible by 4, 3, or 2");
    }
}

fn test_for_demo1() {
    // 从 1 到 5（包含 5）循环
    for i in 1..=5 {
        println!("i: {}", i);
    }
}

fn test_for_demo2() {
    // 从 1 到 4（不包含 4）循环
    for i in 1..4 {
        println!("i: {}", i);
    }
}

// 获取元素的索引
fn test_for_demo3() {
    let a = [10, 20, 30, 40, 50];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    for (index, value) in a.iter().enumerate() {
        println!("index: {}, value: {}", index, value);
    }
}

// 两种循环方式优劣对比
fn test_for_demo4() {
    let collection = [10, 20, 30, 40, 50];
    // 第一种
    println!("第一种 start");
    for i in 0..collection.len() {
        let item = &collection[i];
        println!("item: {}", item);
    }
    println!("第一种 end");

    // 第二种
    println!("第二种 start");
    for item in collection {
        println!("item: {}", item);
    }
    println!("第二种 end");

    //  使用迭代器
    println!("第三种 start");
    for item in collection.iter() {
        print!("item: {}", item);
    }
}

// continue 和 break
fn test_for_demo5() {
    println!("continue start");
    for i in 1..4 {
        if i == 2 {
            continue;
        }
        println!("i: {}", i);
    }

    println!("continue end");
    println!("break start");
    for i in 1..4 {
        if i == 2 {
            break;
        }
        println!("i: {}", i)
    }
    println!("break end");
}

fn test_while() {
    let mut n = 0;
    while n <= 5 {
        println!("n: {}", n);
        n += 1;
    }
    println!("while end");
}

fn test_loop() {
    let mut n = 0;
    loop {
        n += 1;
        if n == 6 {
            break;
        }
        println!("n: {}", n);
    }
    println!("n loop end");

    let mut n1 = 0;
    loop {
        if n1 > 6 {
            break;
        }
        println!("n1: {}", n1);
        n1 += 1;
    }

    println!("n1 loop end");
}
