// clone required for Clone
#[derive(Debug, Clone)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let mut v = vec![1, 2, 3];
    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 10;
    }

    v.push(4);
    println!("{:?}", v);

    let add_vec = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];

    for i in add_vec {
        // show_addr(i);
        let val = i.clone();
        show_addr(val);
    }
}

fn show_addr(ip: IpAddr) {
    println!("{:?}", ip);
}
//
// pub trait Clone: Sized {
//     fn clone(&self) -> Self;
//     fn clone_from(&mut self, source: &Self) {
//         *self = source.clone();
//     }
// }
