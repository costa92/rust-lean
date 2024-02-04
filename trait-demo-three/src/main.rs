
struct Point<T> {
    x: T,
    y: T,
}
fn print<T: std::fmt::Display>(p: Point<T>) {
    println!("x = {}, y = {}", p.x, p.y);
}

trait TraitA {
    const LEN: u32 = 10;
}

struct StructA;
impl TraitA for StructA{
    const LEN: u32 = 20;
}

trait Shape {
    fn area(&self) -> f64;
}

// trait Circle : Shape {
//     fn radius(&self) -> f64;
// }

trait Circle where Self: Shape {
    fn radius(&self) -> f64;
}

struct CircleStruct;
impl Shape for CircleStruct {
    fn area(&self) -> f64 {
        3.14
    }
}

impl Circle for CircleStruct {
    fn radius(&self) -> f64 {
        println!("2"); 
        1.0 

    }
}  

impl CircleStruct {
    fn print(&self) {
        println!("area = {}, radius = {}", self.area(), self.radius());
    }
}


mod module_a {

    pub trait Shape {
        fn play(&self) {
            println!("play in module_a");
        }
    }
    pub struct A;
    impl Shape for A {
        fn play(&self) {
            println!("play in A");
        }
    }
}

mod module_b {
    use super::module_a::Shape; // 使用父模块的模块
    use super::module_a::A; // 使用父模块的模块

    pub(crate) fn doit(){
        let a = A;
        a.play();
    }
}




fn main() {


    
    let p = Point { x: 5, y: 10 };
    print(p);
    let p = Point { x: 1.0, y: 4.0 };
    print(p);
    let p = Point { x: "hello", y: "world" };
    print(p);

    println!("{}", StructA::LEN);
    println!("{}", <StructA as TraitA>::LEN);

    let c = CircleStruct;
    c.print();

    <CircleStruct as Circle>::radius(&c);
    <CircleStruct as Shape>::area(&c);

    module_b::doit();
}

