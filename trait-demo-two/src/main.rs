pub trait Summary {
    fn summarize_authore(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_authore())
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize_authore(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notift(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Summary 约束类型
pub fn notift_t<T: Summary>(item: &T) {
    println!("t Breaking news! {}", item.summarize());
}

fn main() {
    let weibo = Weibo {
        username: "张三".to_string(),
        content: "今天天气真好".to_string(),
    };
    println!("{}", weibo.summarize());

    notift(&weibo);
    notift_t(&weibo)
}
