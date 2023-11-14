fn main() {
    let mut v = Vec::new();
    v.push(1);

    println!("{:?}", v);

    let v2 = vec![1, 2, 3];

    let third: &i32 = &v2[2];
    println!("The third element is {}", third);

    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let does_not_exist = &v2[100];
    println!("{:?}", does_not_exist);
    let does_not_exist = v2.get(100);
    println!("{:?}", does_not_exist);

    let mut v3 = vec![1, 2, 3, 4, 5];
    let first = &v3[0];

    v3.push(6);

    println!("The first element is: {}", first);

    let v4 = vec![100, 32, 57];
    for i in &v4 {
        println!("{}", i);
    }
}
