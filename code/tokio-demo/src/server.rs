use std::env;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从程序第一个参数获取地址，或者使用默认地址 127.0.0.1:8888
    let server_addr = env::args().nth(1).unwrap_or("127.0.0.1:8888".to_string());
    // .await? 和 .await 区别
    let listener = TcpListener::bind(&server_addr).await?;
    //死循环，一直在等待客户端请求
    loop {
        // 等待客户端连接
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            let mut offset = 0;
            loop {
                let n = socket.read(&mut buf[offset..]).await.expect("从 socket 读取数据失败");
                if n == 0 {
                    return;
                }
                let client_info = socket.peer_addr().unwrap();
                println!("peer_addr:{} offset:{offset} n=>{n}", client_info);
                let end = offset + n;
                if let Ok(directive) = std::str::from_utf8(&buf[..end]) {
                    println!("directive: {directive}");
                    let output = process(directive, &*client_info.to_string()).await;
                    println!("{output}");
                    socket.write_all(&output.as_bytes()).await.expect("向 socket 写入数据失败");
                } else {
                    offset = end;
                }
            }
        });
    }
}

async fn process(directive: &str, client_info: &str) -> String {
    if directive == "gettime" {
        let output = Command::new("date").output().await.unwrap();
        String::from_utf8(output.stdout).unwrap() +" "+client_info
    } else {
        "错误指令".to_owned()
    }
}
