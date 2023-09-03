use num::complex::Complex;
use std::fmt::Debug;

fn main() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 12.2);
    let c = a + b;
    println!("{} + {}i", c.re, c.im);

    let sum = add(1, 2);
    println!("{}", sum);

    another_function(5, 6.1);

    let x = plus_five(5);

    println!("The value of x is: {}", x);

    let s = plus_or_minus(6);
    println!("The value of s is: {}", s);

    let r = report(1);
    println!("The value of r is: {:?}", r);

    let c = clear(&mut String::from("Hello"));
    println!("The value of c is: {:?}", c);
    let a = addv1(1, 2);
    println!("The value of a is: {}", a);

    dead_end();
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn another_function(x: i32, y: f64) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn plus_five(x: i32) -> i32 {
    x + 5
}

fn plus_or_minus(x: i32) -> i32 {
    if x > 5 {
        return x - 5;
    }
    x + 5
}

fn report<T: Debug>(item: T) {
    println!("{:?}", item);
}

fn clear(text: &mut String) -> () {
    *text = String::from("");
}

fn addv1(x: u32, y: u32) -> u32 {
    return x + y;
}

fn dead_end() -> ! {
    panic!("This function never returns!");
}
