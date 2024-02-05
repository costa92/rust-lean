
struct Atype;
struct Btype;
struct Ctype;


trait TraitA{}

impl TraitA for Atype{}
impl TraitA for Btype{}
impl TraitA for Ctype{}



/*  dyn trait 本身的尺寸在编译期是未知的,所以 dyn trait 的出现总是要借助于引用或智能指针,而 Box 是最常见的，甚至比 &dyn trait 更常见。
原因就是 Box 拥有所有权，这就是 Box 方便的地方，而 &dyn Trait 不拥有所有权，有的时候就没那么方便
*/
fn doit(x: Box<dyn TraitA>){}

fn run_doit(){
    let a = Atype;
    doit(Box::new(a));

    let b = Btype;
    doit(Box::new(b));

    let c = Ctype;
    doit(Box::new(c));
}

struct MyStruct {
    x: Box<dyn TraitA>  //  这里的x是Box<dyn TraitA>类型

    // x: &dyn TraitA // 结构体字段类型是 &dyn TraitA
}

fn run_mystruct(){
    let a = Atype;
    let t1 = MyStruct{x: Box::new(a)};
    
    let b = Btype;
    let t2 = MyStruct{x: Box::new(b)};
    
    let c = Ctype;
    let t3 = MyStruct{x: Box::new(c)};
}

// arc<T>
#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}


impl Point {
    fn play(&self) {
        println!("I'm am play method of Point");
    }

    fn play_ref(&self) {
        println!("I'm am play_ref method of Point");
    }
    fn play_mutref(&mut self) {
        println!("I'm am play_mutref method of Point");
    }

    fn play_own(self) {
        println!("I'm am play_own method of Point");
    }

    fn play_boxown(self: Box<Self>) {  // 这里的self是Box<Self>类型
        println!("I'm am play_boxown method of Point");
    }

    fn play_arcown(self: std::sync::Arc<Self>) { // 这里的self是Arc<Self>类型
        println!("I'm am play_arcown method of Point");
    }
}


fn run_arc(){
    use std::sync::Arc;
    let arced: Arc<Point> = Arc::new(Point { x: 10, y: 20 });
    let another_arced = arced.clone();  // 克隆
    println!("arced {:?}", arced);
    println!("another_arced {:?}", another_arced);

    arced.play();
    another_arced.play();

    let arc3_ref = &another_arced;
    arc3_ref.play();
}


fn run_arc_box(){
    use std::sync::Arc;
    let mut boxed: Box<Point> = Box::new(Point { x: 10, y: 20 });
    boxed.play_ref();
    boxed.play_mutref();    
    boxed.play_boxown();  // play_boxown() 和 play_own() 只能同时打开一个，这两个方法调用都会消耗所有权，导致没法调用另外一个。

    let arced: Arc<Point> = Arc::new(Point { x: 10, y: 20 });
    arced.play_ref();
    // arced.play_mutref(); // 不能用
    // arced.play_own(); // 不能用，Arc 中的T无法被移出
    arced.play_arcown();  // play_arcown() 和 play_own() 只能同时打开一个，这两个方法调用都会消耗所有权，导致没法调用另外一个。

}

fn main() {
    // run_doit();
    // run_mystruct();
    // run_arc();
    run_arc_box();
}