use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    let msg = b"Hello, world!";
    stream.write_all(msg).await?;
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await?;
    println!("{}", String::from_utf8_lossy(&buf[..n]));
    Ok(())
}
