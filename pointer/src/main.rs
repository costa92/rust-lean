fn main() {
    move_stack();
    move_point();
    play_point();
    write_point();
    cloned_point();

    foo(Box::new(Point { x: 10, y: 20 }));
    bar();
}

// Path: pointer/src/main.rs
// Box 实例上使用 解引用运算符*，把里面的堆上的值再次移动回到栈上
fn move_stack() {
    let x: Box<u8> = Box::new(5);
    let y: u8 = *x;   // 这里这个val整数实例就是在栈上的值
    assert_eq!(5, y);
    // assert_eq!(5, x); // 这里编译器会报错，因为x已经被移动了
    println!("{:?}", x);
    println!("{:?}", y);
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
fn move_point(){
    let p = Point { x: 10, y: 20 };
    let boxed: Box<Point> = Box::new(p);
    let val: Point = *boxed; // 这里这个val整数实例就是在栈上的值

    println!("{:?}", val);
    // println!("{:?}", boxed); // 这里编译器会报错，因为x已经被移动了
}

impl Point {
    fn play(&self) {
        println!("I'm am a method of Point");
    }
}

fn play_point() {
    let boxed: Box<Point> = Box::new(Point { x: 10, y: 20 });
    boxed.play();  // 点操作符触发deref
    println!("{:?}", boxed);
}


fn write_point(){
    let mut boxed: Box<Point> = Box::new(Point { x: 10, y: 20 });
    *boxed = Point { x: 20, y: 30 };  //  这一行，使用解引用操作更新值

    println!("{:?}", boxed);
}

#[warn(unused_mut)]
fn cloned_point(){
    let boxed: Box<Point> = Box::new(Point { x: 10, y: 20 });
    let mut another_boxed = boxed.clone();  // 克隆
    *another_boxed = Point { x: 100, y: 200 };  //  这一行，使用解引用操作更新值

    println!("cloned_point:{:?}", boxed);  // 这里的值不会被改变
    println!("cloned_point:{:?}", another_boxed); // 这里的值会被改变
}

// Box 作为函数参数
fn foo(p:Box<Point>){  // 这里的p是Box<Point>类型
    println!("foo {:?}", p);
}


//Box<T> 本身作为一种类型，对它做引用操作当然是可以的
fn bar(){
  let boxed: Box<Point> = Box::new(Point { x: 10, y: 20 });
  boxed.play();  // 调用类型方法
  
  let y = &boxed;  // 这里的y是Box<Point>类型的引用
  y.play();  // 调用类型方法
  println!("bar {:?}", y);
}
