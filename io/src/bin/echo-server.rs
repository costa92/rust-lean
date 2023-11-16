use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:6371").await?;
    let (mut rd, mut wr) = io::split(socket);
    // 创建异步任务，在后台写入数据
    tokio::spawn(async move {
        wr.write_all(b"hello\n").await?;
        wr.write_all(b"world\n").await?;

        // 有时，需要给予 Rust 一些类型暗示，才能正确的推导出类型
        Ok::<_, io::Error>(())
    });
    let mut buf = vec![0; 128];
    loop {
        let n = rd.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        println!("GOT {:?}", &buf[..n]);
    }
    Ok(())
}
