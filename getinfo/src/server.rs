use std::env;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::process::Command;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use bytes::Bytes;
use futures::{SinkExt, StreamExt};

#[tokio::main]
async fn main() ->Result<(), Box<dyn std::error::Error>> {
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());
    println!("Listening on: {}", addr);
    let listening = TcpListener::bind(&addr).await?;
    // 通过循环接受连接
    loop {
        // 接受连接
        // let (mut socket, _ ) = listening.accept().await?;
        let (socket, _ ) = listening.accept().await?;

        // 包裹成一个Frame stream
        let mut framed_stream = Framed::new(socket,LengthDelimitedCodec::new());
        // 处理连接
        tokio::spawn(async move {
            // // 分配一个新的buffer
            // let mut buf = vec![0; 1024];
            // let mut offset = 0;
            // loop {
            //     // 读取数据
            //     let n = socket.read(&mut buf[offset..]).await.expect("failed to read data from socket");
            //     // 如果读取的数据为0，说明连接已经关闭
            //     if n == 0 {
            //         return;
            //     }
            //     // 打印接收到的数据
            //     println!("offset: {offset}, n: {n}");
            //    let end = offset +n;
            //     // 转换指令为字符串 
            //     if let Ok(s) = std::str::from_utf8(&buf[..end]) {
            //         println!("Received: {}", s);

            //         // 处理指令
            //         let output = process(s).await;
            //         println!("Processed: {}", output);
            //         // 发送处理结果
            //         socket.write_all(&output.as_bytes()).await.expect("failed to write data to socket");
            //     }else {
            //         // 判断是否转换失败，如果失败，就有可能是网络上的数据还没读完
            //        // 要继续loop读下一波数据     
            //           offset = end;
            //     }
            // }

            // v2
            while let Some(msg) = framed_stream.next().await {
              match msg {
                Ok(msg) => {
                        let directive = String::from_utf8(msg.to_vec()).expect("invalid utf8");
                        println!("Received: {}", directive);
                        let output = process(&directive).await;
                        println!("Processed: {}", output);
                       _ = framed_stream.send(Bytes::from(output)).await;
                    }
                Err(e) => {
                    println!("failed to read from socket; error = {:?}", e);
                    return;
                    }
                }
            }
         });
    }
}


async fn process(directive: &str) ->String {
        println!("processing directive: {}", directive.trim());
        if directive.trim() == "gettime" {
            // 这里我们用了unwrap()是因为我们一般确信执行date命令不会失败 
            // 更可靠的做法是对返回的Result作处理 
            let output = Command::new("date").output().await.unwrap(); 
            String::from_utf8(output.stdout).unwrap()
        } else {
            "unknown directive".to_owned()
        }
}