use tokio::task;
use std::time::Duration;
use tokio::time;
use tokio::sync::{mpsc, oneshot};
#[tokio::main]
async fn main() {
    // channel_one().await;
    // channel_sleep().await;
    oneshot_channel().await;
}


async fn channel_sleep(){
    let mut db: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let (tx,mut rx) = mpsc::channel::<u32>(100); // 创建一个channel

    let tx1 = tx.clone(); //  克隆
    let tx2 = tx.clone(); // 克隆


    let task_a = task::spawn(async move {
        println!("task_a");
       time::sleep(Duration::from_secs(3)).await;
        if let Err(_) = tx1.send(50).await {  // 发送数据
            println!("task_a send error");
            return;
        }
    });


    let task_b = task::spawn(async move {
        println!("task_b");
        // time::sleep(Duration::from_secs(1)).await;
        if let Err(_) = tx2.send(100).await {  // 发送数据
            println!("task_b send error");
            return;
        }
    });


    let task_c = task::spawn(async move {
        println!("task_c");
        while let Some(data) = rx.recv().await {  // 接收数据
           println!("task_c received: {}",data);
           db[4] = data;
           println!("task_c db: {:?}",db);
        }
    });
    
    _ = task_c.await.unwrap(); // task_c 放在前面来await
     _ = task_a.await.unwrap();   // 并没有执行到这里
     _ = task_b.await.unwrap();
}

async fn channel_one(){
    let mut db: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let (tx,mut rx) = tokio::sync::mpsc::channel::<u32>(100); // 创建一个channel


    let tx1 = tx.clone(); //  克隆
    let tx2 = tx.clone(); // 克隆

    let task_a = task::spawn(async move {
        println!("task_a");
        if let Err(_) = tx1.send(50).await {  // 发送数据
            println!("task_a send error");
            return;
        }
    });

    let task_b = task::spawn(async move {
        println!("task_b");
        if let Err(_) = tx2.send(100).await {  // 发送数据
            println!("task_b send error");
            return;
        }
    });

    
    let task_c = task::spawn(async move {
        println!("task_c");
        while let Some(data) = rx.recv().await {  // 接收数据
           println!("task_c received: {}",data);
           db[4] = data;
           println!("task_c db: {:?}",db);
        }
    });

    task_c.await.unwrap();
    task_b.await.unwrap();
    task_a.await.unwrap();
}


async fn oneshot_channel(){
    let mut db: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let (tx,mut rx) = mpsc::channel::<(u32,oneshot::Sender<bool>)>(100);

    let tx1 = tx.clone();
    let tx2 = tx.clone();

    let task_a = task::spawn(async move {
        println!("task_a");
        time::sleep(Duration::from_secs(3)).await;
        let (resp_tx,resp_rx) = oneshot::channel();
        if let Err(_) = tx1.send((50,resp_tx)).await {  // 发送数据
            println!("task_a send error");
            return;
        }
        // 接收数据
        if let Ok(data) = resp_rx.await {
           if data {
               println!("task_a received: {}",data);
               println!("task_a finished with success.");
           } else {
                println!("task_a received: {}",data);
                println!("task_a finished with error.");
           }
        }else {
            println!("task_a received error");
            return;
        }
    });

    let task_b = task::spawn(async move {
        println!("task_b");
        let (resp_tx,resp_rx) = oneshot::channel();
        if let Err(_) = tx2.send((100,resp_tx)).await {  // 发送数据
            println!("task_b send error");
            return;
        }
        // 接收数据
        if let Ok(data) = resp_rx.await {
           if data {
               println!("task_b received: {}",data);
               println!("task_b finished with success.");
           } else {
                println!("task_b received: {}",data);
                println!("task_b finished with error.");
           }
        }else {
            println!("task_b received error");
            return;
        }
    });


    let task_c = task::spawn(async move {
        println!("task_c");
        while let Some((data,resp_tx)) = rx.recv().await {  // 接收数据
           println!("task_c received: {}",data);
           db[4] = data;
           println!("task_c db: {:?}",db);
           resp_tx.send(true).unwrap();
        }
    });
    task_a.await.unwrap();    
    task_b.await.unwrap();
    task_c.await.unwrap();
}