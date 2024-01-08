use std::env;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从程序第一个参数获取地址，或者使用默认地址 127.0.0.1:8888
    let server_addr = env::args().nth(1).unwrap_or("127.0.0.1:8888".to_string());
    // 连接服务器
    let mut stream = TcpStream::connect(&server_addr).await?;

    stream.write_all(b"gettime").await?;

    let mut buf: Vec<u8> = Vec::with_capacity(8128);
    let mut resp = [0u8; 2048];
    loop {
        let n = stream.read(&mut resp).await?;
        buf.extend_from_slice(&resp[0..n]);
        if n == 0 {
            panic!("连接断开")
        } else if buf.len() >= 28 {
            break;
        } else { continue; }
    }
    let timeinfo = String::from_utf8(buf)?;
    print!("{}", timeinfo);

    Ok(())
}