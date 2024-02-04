fn main() {
    // 解析 Option 和 Result
    let s = String::from("result_one");
    println!("{}",s);
    let a: Option<String> = Some(s);
    println!("{}",a.unwrap());

    let x = Some("value");
    assert_eq!(x.expect("fruits are healthy"), "value");
    println!("{}",x.expect("fruits are healthy"));


    // result
    // let path = std::env::var("IMPORTANT_PATH").expect("env variable `IMPORTANT_PATH` should be set by `wrapper_script.sh`");
    // println!("start");
    // println!("{}",path)

    let x_one = Some("air");
    assert_eq!(x_one.unwrap(),"air");
    println!("{}",x_one.unwrap());

    let x_two: Result<u32,&str> = Ok(2);
    assert_eq!(x_two.unwrap(),2);
    println!("{}",x_two.unwrap());

    // Option
    assert_eq!(Some("car").unwrap_or("bike"),"car");
    assert_eq!(None.unwrap_or("bike"),"bike");

    // Result
    let default = 2;
    let x: Result<u32,&str> = Ok(9);
    assert_eq!(x.unwrap_or(default),9);


    let x: Result<u32,&str> = Err("error");
    assert_eq!(x.unwrap_or(default),2);


    // Option
    let x:Option<u32> =None;
    let y:Option<u32> = Some(12);

    assert_eq!(x.unwrap_or_default(),0);
    assert_eq!(y.unwrap_or_default(),12);


    // Result
    let good_year_from_input = "1909";
    let bad_year_from_input= "190blarg";

    // let good_year = good_year_from_input.parse().unwrap_or_default();
    let good_year = good_year_from_input.parse::<i32>().unwrap_or_default();
    let bad_year= bad_year_from_input.parse::<i32>().unwrap_or_default();
    // let bad_year = bad_year_from_input.parse().unwrap_or_default();

    assert_eq!(good_year,1909);
    assert_eq!(bad_year,0);


    // 不解析 Option 和 Result
    // map
    let maybe_some_string = Some(String::from("Hello, World!"));
    let maybe_some_len = maybe_some_string.map(|s| s.len());

    assert_eq!(maybe_some_len,Some(13));

    let x: Option<String> = None;
    assert_eq!(x.map(|v| v.len()),None);


    // cloned() 通过克隆 Option 里面的内容，把 Option<&T> 转换成 Option。
    let x = 12;
    let opt_x = Some(&x);
    assert_eq!(opt_x,Some(&12));

    // &T -> T
    let cloned_x = opt_x.cloned();
    assert_eq!(cloned_x,Some(12));


    // is_some()  如果 Option 是 Some 值，返回 true。
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_some(),true);

    let x: Option<u32> = None;
    assert_eq!(x.is_some(),false);

    // is_none() 如果 Option 是 None 值，返回 true。
    let x: Option<u32> = None;
    assert_eq!(x.is_none(),true);

    let x: Option<u32> = Some(2);
    assert_eq!(x.is_none(),false);

    // as_ref() 通过引用返回 Option 的内容，如果是 Some 值，返回 Some(&T)。
    let text: Option<String> = Some("Hello, World!".to_string());
    let text_ref: Option<&String> = text.as_ref();
    assert_eq!(text_ref,Some(&String::from("Hello, World!")));  

    let text_length: Option<usize> = text_ref.map(|s| s.len());
    println!("{:?}",text_length);

    // as_mut() 通过可变引用返回 Option 的内容，如果是 Some 值，返回 Some(&mut T)。
    let mut x = Some(2);
    match x.as_mut() {
        Some(v) => *v = 42,
        None => {},
    }
    assert_eq!(x,Some(42));

    // take() 从 Option 中取出 Some 值，留下 None。
    let mut x = Some(2);
    let y = x.take();

    assert_eq!(x,None);
    assert_eq!(y,Some(2));

    let mut x: Option<u32> = None;
    let y = x.take();

    assert_eq!(x,None);
    assert_eq!(y,None);

    // replace() 用新值替换 Option 的内容，返回 Option 里面的旧值。

    let mut x = Some(2);
    let old = x.replace(5);

    assert_eq!(x,Some(5));
    assert_eq!(old,Some(2));

    let mut x: Option<u32> = None;
    let old = x.replace(3);
    assert_eq!(x,Some(3));
    assert_eq!(old,None);

    // and_then() 如果 Option 是 Some 值，返回闭包里面的结果，否则返回 None。
    
    fn sq_then_to_string(x: u32) -> Option<String> {
        x.checked_mul(x).map(|n| n.to_string())
        // checked_mul 检查乘法是否溢出 
    }

    assert_eq!(Some(2).and_then(sq_then_to_string),Some("4".to_string()));
    assert_eq!(Some(1_000_000).and_then(sq_then_to_string),None);
    assert_eq!(None.and_then(sq_then_to_string),None);


    // Result
    // map
    let line = "1\n2\n3\n";
/*     for num in line.lines() {
        println!("{}",num.parse::<i32>().unwrap());
    }
 */
    for num in line.lines() {
      match num.parse::<i32>().map(|n| n*2){
          Ok(n) => println!("{}",n),
          Err(e) => println!("Error: {}",e),
      }
    }   

    // is_ok() 如果 Result 是 Ok 值，返回 true。

    let x: Result<u32,&str> = Ok(2);
    assert_eq!(x.is_ok(),true);

    let x: Result<u32,&str> = Err("Nothing here");
    assert_eq!(x.is_ok(),false);


    // is_err() 如果 Result 是 Err 值，返回 true。
    let x: Result<u32,&str> = Err("Nothing here");
    assert_eq!(x.is_err(),true);

    let x: Result<u32,&str> = Ok(2);
    assert_eq!(x.is_err(),false);

    // as_ref() 通过引用返回 Result 的内容，如果是 Ok 值，返回 Ok(&T)。
    let x: Result<u32,&str> = Ok(2);
    assert_eq!(x.as_ref(),Ok(&2));

    let x: Result<u32,&str> = Err("Nothing here");
    assert_eq!(x.as_ref(),Err(&"Nothing here"));


    // as_mut() 通过可变引用返回 Result 的内容，如果是 Ok 值，返回 Ok(&mut T)。
    fn mutate(r: &mut Result<i32,i32>) {
        match r.as_mut() {
            Ok(v) => *v = 42,
            Err(e) => *e = 0,
        }
    }
    let mut x: Result<i32,i32> = Ok(5);
    mutate(&mut x);
    assert_eq!(x,Ok(42));

    let mut x: Result<i32,i32> = Err(13);
    mutate(&mut x);
    assert_eq!(x.unwrap_err(),0);   

    // and_then() 如果 Result 是 Ok 值，返回闭包里面的结果，否则返回 Err。

    fn sq_then_to_string_rs(x: u32) -> Result<String,&'static str> {
        x.checked_mul(x).map(|n| n.to_string()).ok_or("result overflow")
    }

    assert_eq!(Ok(2).and_then(sq_then_to_string_rs),Ok("4".to_string()));
    assert_eq!(Ok(1_000_000).and_then(sq_then_to_string_rs),Err("result overflow"));
    assert_eq!(Err("Nothing here").and_then(sq_then_to_string_rs),Err("Nothing here"));


    // map_err() 如果 Result 是 Err 值，返回闭包里面的结果，否则返回 Ok。
    fn stringify(x: u32) -> String{
        format!("error code: {}",x)
    }
    let x: Result<u32,u32> = Ok(3);
    // assert_eq!(x.map_err(stringify),Ok(2));  
    println!("{:?}",x.map_err(stringify));

    let x: Result<u32,u32> = Err(3);
    assert_eq!(x.map_err(stringify),Err("error code: 3".to_string()));


    // Option 和 Result 转换
    // ok_or() 如果 Option 是 Some 值，返回 Some 值，否则返回 Err。
    let x: Option<u32> = Some(2);
    assert_eq!(x.ok_or("Nothing here"),Ok(2));

    let x: Option<u32> = None;
    assert_eq!(x.ok_or("Nothing here"),Err("Nothing here"));

    // Result 转换 Option  使用 ok()
    // ok() 如果 Result 是 Ok 值，返回 Some 值，否则返回 None。
    let x: Result<u32,&str> = Ok(2);
    assert_eq!(x.ok(),Some(2));

    let x: Result<u32,&str> = Err("Nothing here");
    assert_eq!(x.ok(),None);

    // Result 转换 Option  使用 err()
    // err() 如果 Result 是 Err 值，返回 Some 值，否则返回 None。
    let x: Result<u32,&str> = Err("Nothing here");
    assert_eq!(x.err(),Some("Nothing here"));
    
    let x: Result<u32,&str> = Ok(2);
    assert_eq!(x.err(),None);


    let mut a = ["1".to_string(), "2".to_string(), "3".to_string()];
    let mut an_iter = a.iter();        
    assert_eq!(Some(&"1".to_string()), an_iter.next());    
    assert_eq!(Some(&"2".to_string()), an_iter.next());    
    assert_eq!(Some(&"3".to_string()), an_iter.next());    
    assert_eq!(None, an_iter.next());    
    
   let mut an_iter = a.iter_mut();        
    assert_eq!(Some(&mut "1".to_string()), an_iter.next());    
    assert_eq!(Some(&mut "2".to_string()), an_iter.next());    
    assert_eq!(Some(&mut "3".to_string()), an_iter.next());    
    assert_eq!(None, an_iter.next());   
     
    let mut an_iter = a.into_iter();        
    assert_eq!(Some("1".to_string()), an_iter.next());    
    assert_eq!(Some("2".to_string()), an_iter.next());    
    assert_eq!(Some("3".to_string()), an_iter.next());    
    assert_eq!(None, an_iter.next());        
    
    println!("{:?}", a); 
}

