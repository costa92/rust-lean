use std::net::IpAddr;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Read;
fn main() {
    let s = "100eee";

    if let Err(e) = s.parse::<i32>() {
        // e 这里是 ParseIntError 类型
        println!("Error: {}", e);
    }

    let addr = "127.0.0.1:8080".parse::<IpAddr>();
    if let Err(e) = addr {
        // e 这里是 AddrParseError 类型
        println!("Error: {}", e);
    }

    foo();
    let res = foo2(0);
    println!("{}",res.expect("error"));    
    // if (res.is_err()){
    //     println!("Error: {}", res.unwrap_err());
    // } else {
    //     println!("Ok: {}", res.unwrap());
    // }
    // 处理错误
//     match res {
//         Ok(s) => println!("Ok: {}", s),
//         Err(e) => println!("Error: {}", e),
//     }
    let _ = read_file();
    let res_file =   read_file_map_err();
    match res_file {
        Ok(s) => println!("Ok: {}", s),
        Err(e) => println!("Error: {}", e),
    }

    let res_file_link = read_file_link_err();
    match res_file_link {
        Ok(s) => println!("Ok: {}", s),
        Err(e) => println!("Error: {}", e),
    }

    let  read_file_map_err_fix = read_file_map_err_fix();
    match read_file_map_err_fix {
        Ok(s) => println!("Ok: {}", s),
        Err(e) => println!("Error: {}", e),
    }

    let  read_file_map_err_fix2 = read_file_map_err_fix2();
    match read_file_map_err_fix2 {
        Ok(s) => println!("Ok: {}", s),
        Err(e) => println!("Error: {:?}", e),
    }
}

// 定义一个错误类型 Error 一般使用 enum 来定义
#[derive(Debug)]
#[allow(dead_code)]
enum HereError {
    ErrorOne,
    ErrorTwo,
    ErrorThree,
}

// 一个函数返回 Err
fn bar() -> Result<i32, HereError> {
    Err(HereError::ErrorThree)
}


fn foo(){
    match bar() {
        Ok(n) => println!("Ok: {}", n),
        Err(e) => match e {
            HereError::ErrorOne => println!("ErrorOne"),
            HereError::ErrorTwo => println!("ErrorTwo"),
            HereError::ErrorThree => println!("ErrorThree"),
        }   
    }  
}

#[derive(Debug)]
struct MyError;

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyError")
    }   
}

impl Error for MyError {}


fn foo2(num: u32) -> Result<String,Box<dyn Error>> {
    match num {
        0 => Ok("hello world!".to_string()),
        _ => {
            let my_error = MyError;
            Err(Box::new(my_error))
        }
    }
}



// map_err 用于将错误转换为另一种错误

fn read_file() -> Result<String, String> {
    match File::open("hello.txt") {
        Ok(mut file) => {
            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

fn read_file_map_err() -> Result<String, String> {
    match File::open("hello.txt").map_err(|err| format!("Error opening file:{}",err)) {
        Ok(mut file) => {
            let mut s = String::new();
            match file.read_to_string(&mut s).map_err(|err| format!("Error reading file:{}",err)) {
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

// Result 链式处理
fn read_file_link_err() ->Result<String,String> {
    File::open("hello.txt").map_err(|err| format!("Error opening file:{}",err)).and_then(|mut file| {
        let mut s = String::new();
        file.read_to_string(&mut s).map_err(|err| format!("Error reading file:{}",err)).map(|_| s)
    })
}

// let ret = a_result?; 等价于 let ret = match a_result { Ok(ret) => ret, Err(e) => return Err(e), // 注意这里有一个return语句。};
fn read_file_map_err_fix() -> Result<String,String>{
    let mut file = File::open("hello.txt").map_err(|err| format!("Error opening file:{}",err))?;
    let mut s = String::new();
    file.read_to_string(&mut s).map_err(|err| format!("Error reading file:{}",err))?;
    Ok(s)
}

#[derive(Debug)]
struct MyError2 (String); // 自定义错误类型

impl From <std::io::Error> for MyError2 {
    fn from(err: std::io::Error) -> Self {
        MyError2(format!("Error:{}",err))
    }
}

fn read_file_map_err_fix2() -> Result<String,MyError2>{
    let mut file = File::open("example.txt")?; 
    let mut contents = String::new(); 
    file.read_to_string(&mut contents)?;
    Ok(contents)
}