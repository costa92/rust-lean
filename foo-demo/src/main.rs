fn main() {
    let rs =   foo(1, "a", "b");
    println!("{}",rs);

    let lrs = longest("a", "b");
    println!("{}",lrs);

    let s1 = String::from("long string is long ");
    let s2 = String::from("xyz");
    let result;
    {
        result = longest(s1.as_str(), s2.as_str());
    }
    println!("The longest string is {}", result);
}

fn foo<'a>(i: u32, a: &'a str , b: &'a str) -> &'a str {
    if i ==1{
        a
    }else{
        b
    }
}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


struct A {
    foo: String,
}

impl A {
    fn play<'a>(&'a self, a: &'a str, b: &str) -> &str {
        // &self.foo
        a
    }
}

