use std::env;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use bytes::Bytes;use futures::{SinkExt, StreamExt};
#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>>{
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());
    println!("Listening on: {}", addr);

   
    // let mut stream = TcpStream::connect(&addr).await?;

    // let directive = "gettime";
    // 发送指令
    // stream.write_all(directive.as_bytes()).await?;

    // // 等待tcp server的回复，读取内容 // 这里用动态数组来存储从服务端返回的内容 
    // let mut buf: Vec<u8> = Vec::with_capacity(8128);

    // // 缓冲区
    // let mut resp = [0u8; 2048];
    // loop {
    //     let n = stream.read(&mut resp).await?;
    //     buf.extend_from_slice(&resp[0..n]);
    //     if n == 0 {
    //         panic!("connection closed by peer");
    //     }else if  buf.len() >= 28 {
    //         // like: "Tue Oct 31 14:56:27 CST 2023" 
    //         // buf 已经填充了足够的内容
    //         break;
    //     }else {
    //         continue;
    //     }
    // }

    // // 打印服务端返回的内容
    // let timeinfo = String::from_utf8(buf)?;
    // println!("timeinfo: {}", timeinfo);


// v2
        let  stream = TcpStream::connect(&addr).await?;
        let mut framed_stream = Framed::new(stream, LengthDelimitedCodec::new());
        let directive = "gettime";
        framed_stream.send(Bytes::from(directive)).await?;

        if let Some(msg) = framed_stream.next().await {
            // if (&msg).is_err() {
            //     return Err("error from server".into());
            // }

            // let timeinfo = String::from_utf8(msg?.to_vec())?;

            let timeinfo = String::from_utf8(msg?.to_vec())?;
            println!("timeinfo: {}", timeinfo);
        }

    Ok(())
}