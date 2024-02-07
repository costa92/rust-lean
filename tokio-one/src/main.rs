use tokio::task;
use tokio::sync::Mutex;
use tokio::sync::RwLock;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;
#[tokio::main]
async fn main() {
    let  db: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let arc = std::sync::Arc::new(Mutex::new(db));
    let arc2 = arc.clone();
    let arc3 = arc.clone();

    let handle2 = task::spawn(async move {
        let mut arc = arc3.lock().await;
        arc.push(11);
        arc[1] = 33;
    });


    let handle = task::spawn(async move  {
        let mut arc = arc2.lock().await;
        arc.push(10);
        arc[1] = 32;
    });

    handle.await.unwrap();
    handle2.await.unwrap();
    println!("arc {:?}", arc.lock().await);



    let lock = RwLock::new(5);
    // 多个读锁可以同时存在
    {
        let r1 = lock.read().await;
        let r2 = lock.read().await;

        println!("r1 = {}, r2 = {}", r1, r2);
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    }

    // 同时只能存在一个写锁
    {
        let mut w = lock.write().await;
        *w += 1;
        assert_eq!(*w, 6);
    } // 释放写锁



    // 原子操作  AtomicBool, AtomicIsize, AtomicUsize, AtomicI8, AtomicU8, AtomicI16, AtomicU16, AtomicI32, AtomicU32, AtomicI64, AtomicU64, AtomicIsize, AtomicUsize
    let atomic = AtomicU32::new(43); // 创建一个原子计数器
    let arc_data = Arc::new(atomic);

    arc_data.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    println!("arc_data {:?}", arc_data);

    // 锁（lock）和无锁（lock-free）是计算机科学领域一个非常大的课题，Rust 有本书 《Rust Atomics and Locks》<https://marabos.nl/atomics/>专门讲这个，有兴趣的话你可以看一看。
    let mut some_var = AtomicU32::new(10);
    // 更新
    *some_var.get_mut() = 5;
    assert_eq!(*some_var.get_mut(), 5);
}


