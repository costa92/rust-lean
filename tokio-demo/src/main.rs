use tokio::fs::File;
use tokio::io::AsyncWriteExt; // 引入AsyncWriteExt trait
use tokio::io::AsyncReadExt;  // 引入AsyncReadExt trait
#[tokio::main] // // 这个是tokio库里面提供的一个属性宏标注  
async fn main() { // 这里的async关键字是异步函数的标志
    let result = doit().await;  // 这里的await关键字是等待异步函数的返回结果
    match result {
        Ok(_) => println!("File written successfully"),
        Err(e) => println!("Error writing file: {:?}", e),
    }

    let result = read_file().await;
    match result {
        Ok(_) => println!("File read successfully"),
        Err(e) => println!("Error reading file: {:?}", e),
    }

    interval().await;


    task_spawn().await;
    // task_spawn_sample().await;
    batch_task_spawn().await;
}

async fn doit()->std::io::Result<()>{
    let mut file = File::create("foo.txt").await.unwrap();  // 创建 文件 foo.txt
    file.write_all(b"hello, world!").await.unwrap();  // 写入内容
    Ok(())
}


async fn read_file() -> std::io::Result<()> {
    let mut file = File::open("foo.txt").await.unwrap();  // 打开文件
    let mut contents = vec![];  // 创建一个可变的空向量
    file.read_to_end(&mut contents).await.unwrap();  // 读取文件内容
    println!("file content: {:?}", contents);  // 打印文件内容
    println!("file content: {:?}", String::from_utf8_lossy(&contents));  // 打印文件内容
    println!("len = {}", contents.len());  // 打印文件内容长度
    Ok(())
}

async fn interval(){
    // 创建 Interval 实例 
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(1000)); // Duration::from_millis(1000) 表示每隔一秒

    // 每隔一秒打印一次
    let mut count = 0;
    loop {
        count += 1;
        interval.tick().await;
        println!("tick");
        if count == 5 {
            break;
        }
    }
}


// 注意：main task 是父task 新创建的task是子task
// 在 async fn 子 task的生存期有可能比父task的生存期长，也就是说子task可能会在父task结束之后才结束

async fn task_spawn(){
    use tokio::task;
    // 创建一个新的task
    let task_a = task::spawn(async {
        "task a"
    });

    // 创建一个新的task
    let task_b = task::spawn(async {
        "task b"
    });

    // 等待子task结束
    let  result_a = task_a.await.unwrap();
    let  result_b = task_b.await.unwrap();
    println!("result_a: {}", result_a);
    println!("result_b: {}", result_b);
}

/* JoinHandler task::spawn() 函数实际有一个返回值，它返回一个 handler，这个 handler 可以让我们在 main task 里管理新创建的 task。这个 handler 也可以用来指代这个新的 task，
 相当于给这个 task 取了一个名字。比如示例里，我们就把这个新的任务命名为 task_a，它的类型是 JoinHandler。在用 spawn() 创建 task_a 后，这个新任务就立即执行。

 task_a.await 会返回一个 Result，所以上面代码中，需要加一个 unwrap() 把 task_a 真正的返回内容解包出来。
 至于对 task 的 .await 为什么会返回一个 Result，而不是直接返回异步任务的返回值本身，是因为 task 里有可能会发生 panic。你可以看一下例子。
 */

// 一个简单的例子
async fn task_spawn_sample(){
    let task_a = tokio::task::spawn(async {
        panic!("task_a panic");
    });

    // 当 task_a 发生 panic 时，对 task handler 的 await 会返回一个 Err
    let result_a = task_a.await;
    match result_a {
        Ok(_) => println!("task_a finished successfully"),
        Err(e) => println!("task_a finished with error: {:?}", e),
    }
}


async fn my_background_task(id:i32) ->String {
    // 一些耗时的操作
    let s = format!("Starting background task {}.",id);
    println!("{}",s);
    s
}

// 批量创建task
async fn batch_task_spawn(){
    use tokio::task;
    let ops = vec![1,2,3];

    let mut task = Vec::with_capacity(ops.len());
    for op in ops {
        task.push(task::spawn(my_background_task(op)));
    }

    let mut results = Vec::with_capacity(task.len());
    for t in task {
        results.push(t.await.unwrap());
    }

    println!("results: {:?}", results);
    println!("{}",results.len());
}