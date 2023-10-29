// 编译器建议我们给 T 添加一个类型限制：std::cmp::PartialOrd，它是可以比较的类型。
fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

// 为泛型 T 的每个实例都定义了一个与之对应的类型 U。
struct Pont<T, U> {
    x: T,
    y: U,
}

impl<T, U> Pont<T, U> {
    fn mixup<V, W>(self, other: Pont<V, W>) -> Pont<T, W> {
        Pont {
            x: self.x,
            y: other.y,
        }
    }
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_lisy = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_lisy);
    println!("The largest char is {}", result);

    let interger = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("interger.x = {}", interger.x);
    println!("float.y = {}", float.y);

    let p = Pont { x: 5, y: 10.4 };
    println!("p.x: {},p.y: {}", p.x, p.y);
    // impl 泛指实现
    let impl_p = Point { x: 5, y: 10 };
    println!("p.x = {}", impl_p.x());

    let p1 = Pont { x: 5, y: 10.4 };
    let p2 = Pont { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
