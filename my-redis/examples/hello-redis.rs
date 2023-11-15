use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6370").await?;

    // set key "hello" with value "world"
    client.set("hello11", "world".into()).await?;

    // client.set("hello", "world".into()).await?;
    // get ket "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    //   let op = say_to_world();
    //   println!("hello");
    //   println!("{}", op.await);

    Ok(())
}
#[allow(dead_code)]
async fn say_to_world() -> String {
    String::from("World")
}
