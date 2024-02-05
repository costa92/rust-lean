
#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}


impl Point {
    fn play(&self) {
        println!("I'm am play method of Point");
    }

    fn play_mutref(&mut self) {
        println!("I'm am play_mutref method of Point");
    }

    fn play_own(self) {
        println!("I'm am play_own method of Point");
    }

    fn play_boxown(self: Box<Self>) {
        println!("I'm am play_boxown method of Point");
    }
}

fn main() {
    let mut boxed: Box<Point> = Box::new(Point { x: 10, y: 20 });
    let y = &mut boxed; // 这里的y是一个可变引用
    y.play();

    println!("{:?}", y); // 修改前的值
    **y = Point { x: 20, y: 30 };  // 注意这里用了二级解引用
    println!("{:?}", y); // 修改后的值

    // 这里的值不会被改变
    println!("{:?}", y.x);
    println!("{:?}", y.y);

    let mut boxed: Box<Point> = Box::new(Point { x: 10, y: 20 });

    boxed.play();
    boxed.play_mutref();
    boxed.play_boxown();  // play_boxown() 和 play_own() 只能同时打开一个，这两个方法调用都会消耗所有权，导致没法调用另外一个。
    // boxed.play_own();
}
