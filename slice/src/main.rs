fn main() {
    let mut s = String::from("Hello World");
    let zlen = first_word(&s);
    println!("zlen = {}", zlen);

    s.clear(); // 清空字符串，使其长度为 0 值为 ""
               // zlen 在此处的值仍然是 5，
               // 但是没有更多的字符串让我们可以有效地应用数值 5。zlen 的值现在完全无效！
    println!("s = {}", s);
    println!("s clear zlen zlen = {}", zlen); // zlen = 5

    let s1 = String::from("Hello World");
    let hello = &s1[0..5];
    let world = &s1[6..11];

    println!("{},{}", hello, world);

    let s_h = String::from("hello");
    // 对于 Rust 的 .. range 语法，如果想要从索引 0 开始，
    // 可以不写两个点号之前的值。换句话说，如下两个语句是相同的：
    let slice_1 = &s_h[0..2];
    println!("{}", slice_1);
    let slice_2 = &s_h[..2];
    println!("{}", slice_2);

    let s_len = String::from("hello");
    let len = s_len.len();

    // 依此类推，如果 slice 包含 String 的最后一个字节，
    // 也可以舍弃尾部的数字。这意味着如下也是相同的：

    let slice_3 = &s_len[3..len];
    println!("{}", slice_3);
    let slice_4 = &s_len[3..];
    println!("{}", slice_4);

    let str = String::from("hello");
    let str_len = str.len();

    let slice_5 = &str[0..str_len];
    let slice_6 = &str[..];

    println!("{}", slice_5);
    println!("{}", slice_6);

    let str_v2 = String::from("hello world ");
    let first_word = first_word_v2(&str_v2);
    println!("{}", first_word);

    let second_word = second_word(&str_v2);

    println!("{}", second_word);

    let a = [1, 2, 3, 4, 5];
    let slice_int = &a[1..3];

    assert_eq!(slice_int, &[2, 3]);
}
//  字符串第一个单词的长度
fn first_word(s: &String) -> usize {
    // 因为需要逐个元素的检查 String 中的值是否为空格，需要用 as_bytes 方法将 String 转化为字节数组
    let bytes = s.as_bytes();
    // 接下来，使用 iter 方法在字节数组上创建一个迭代器：
    for (i, &item) in bytes.iter().enumerate() {
        // 在 for 循环中，我们通过字节的字面量语法来寻找代表空格的字节。
        // 如果找到了一个空格，返回它的位置。否则，使用 s.len() 返回字符串的长度：
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_v2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
#[warn(dead_code)]
fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut second_index = 0;
    let mut count = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if count == 1 {
            println!("{}", second_index);
            if item == b' ' {
                println!("{}", i);
                return &s[second_index + 1..i];
            } else {
                let next = &s[i..i + 1];
                println!("{}", next);
            }
        } else if item == b' ' {
            count += 1;
            second_index = i;
        }
    }
    &s[..]
}
