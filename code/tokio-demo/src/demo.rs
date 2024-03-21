use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;
use tokio::time;

#[tokio::main]
async fn main() {
    arc_mutex().await;
    channel().await;
    join().await;
    for i in 0..10 {
        select().await;
    }
}

// select 等待最快的那个执行完成
async fn select() {
    let task_a = tokio::spawn(async move {
        time::sleep(Duration::from_secs(3)).await;
        1
    });
    let task_b = tokio::spawn(async move {
        2
    });

    let task_c = tokio::spawn(async move {
        3
    });
    let ret = tokio::select! {
        r=task_a=>r.unwrap(),
        r=task_b=>r.unwrap(),
        r=task_c=>r.unwrap()
    };
    println!("ret {:?}", ret);
}

// join 等待所有执行完成（花费时间是最长时间那个）
async fn join() {
    println!("{:?}", std::time::SystemTime::now());
    let task_a = tokio::spawn(async move {
        println!("task_a started");
        time::sleep(Duration::from_secs(3)).await;
        println!("task_a end");
        1
    });
    let task_b = tokio::spawn(async move {
        println!("task_b started");
        time::sleep(Duration::from_secs(2)).await;
        println!("task_b end");
        2
    });

    let task_c = tokio::spawn(async move {
        println!("task_c started");
        time::sleep(Duration::from_secs(1)).await;
        println!("task_c end");
        3
    });
    let (r1, r2, r3) = tokio::join!(task_a, task_b, task_c);
    println!("r1: {:?}, r2: {:?}, r3: {:?}", r1, r2, r3);
    println!("{:?}", std::time::SystemTime::now());
}

/// channel
async fn channel() {
    let mut db: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // mpsc Multi-Producers Single Consumer 多生产者单消费者
    // tx 表示发送端，rx 表示接收端  100 代表通道大小
    // mpsc::channel 有背压功能，当接收端不及时接收消息时，发送端会阻塞
    // mpsc::unbounded_channel() 创建没有容量上线的 channel
    // Oneshot Channel 这个通道只能用一次，也就是说只能发送一条数据，发送完之后就关闭了，对应的 tx 和 rx 就无法再次使用了。这个很适合等待计算结果返回的场景
    // broadcast channel 广播模式 多生产者多消费者模式
    // watch channel 一个特定化版本的 broadcast 通道，它只能有一个生产者，但是可以有多个消费者，只关心最后一个值，配置更新需要通知工作任务重新加载，平滑关闭任务等
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    // tx 是一个 Arc 对象
    let tx1 = tx.clone();
    let tx2 = tx.clone();
    // 通过它创建的异步任务，一旦创建好，就会立即扔到 tokio runtime 里执行，不需要对其返回的 JoinHandler 进行 await 才驱动执行
    // task::spawn() 创建的多个任务之间，本身就是并发执行的关系
    let task_a = tokio::spawn(async move {
        if let Err(_) = tx1.send(50).await {
            println!("tx1 发送失败");
            return;
        }
    });
    let task_b = tokio::spawn(async move {
        if let Err(_) = tx2.send(100).await {
            println!("tx2 发送失败");
            return;
        }
    });

    let task_c = tokio::spawn(async move {
        // .await 是一种不消耗资源的等待，tokio 保证这种等待不会让一个 CPU 忙空转。
        while let Some(i) = rx.recv().await {
            println!("task_c 接收到消息: {}", i);
            db.push(i);
            println!("{:?}", db);
        }
    });
    _ = task_a.await.unwrap();
    _ = task_b.await.unwrap();
}


/// 互斥锁
async fn arc_mutex() {
    let db: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let arc_db = Arc::new(Mutex::new(db));
    // 下面的 clone 只是嗯你感觉 Arc 的引用计数
    let arc_db1 = arc_db.clone();
    let arc_db2 = arc_db.clone();

    let task_a = tokio::spawn(async move {
        // 获取锁
        let mut db = arc_db.lock().await;
        db.push(11);
        println!("task_a: {:?}", db);
    });

    let task_b = tokio::spawn(async move {
        // 获取锁
        let mut db = arc_db1.lock().await;
        db.push(20);
        println!("task_b: {:?}", db);
    });
    _ = task_a.await.unwrap();
    _ = task_b.await.unwrap();
    println!("main: {:?}", arc_db2);
}