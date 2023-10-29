#[derive(Debug)]
enum MyEnum {
    Foo,
    Bar,
}

// Rust 标准库中提供了一个非常实用的宏：matches!，它可以将一个表达式跟模式进行匹配，然后返回匹配的结果 true or false。
fn main() {
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    println!("v = {:?}", v);

    // v.iter().filter(|x| x == MyEnum::Foo);  // 因为你无法将 x 直接跟一个枚举成员进行比较
    //
    let res = v.iter().filter(|x| matches!(x, MyEnum::Foo));
    println!("res = {:?}", res);

    let age = Some(30);

    println!("age after = {:?}", age);

    if let Some(age) = age {
        println!("age = {:?}", age);
    }
    println!("age befter = {:?}", age);
}

#[test]
fn test_matches_one() {
    let foo = "f";
    assert!(matches!(foo, "f"));

    assert!(foo.len() == 1 && (foo >= "A" && foo <= "Z" || foo >= "a" && foo <= "z"));

    // assert!(matches!(foo, "A"..="Z" | "a"..="z"));

    // assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));
    //
    //

    let bar = Some(4);

    assert!(matches!(bar, Some(x) if x > 2 ));
}
