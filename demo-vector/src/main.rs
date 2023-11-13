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
}
