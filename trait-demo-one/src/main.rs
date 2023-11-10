pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}

pub struct Post {
    pub title: String,
    pub auther: String,
    pub content: String,
}

impl Summary for Post {}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} 发表了微博{}", self.username, self.content)
    }
}

fn main() {
    let post = Post {
        title: String::from("Rust 语言介绍"),
        auther: String::from("Sunface"),
        content: String::from("Rust var good language"),
    };

    let weibo = Weibo {
        username: "张三".to_string(),
        content: "今天天气真好".to_string(),
    };

    println!("{}", post.summarize());
    println!("{}", weibo.summarize());
}
