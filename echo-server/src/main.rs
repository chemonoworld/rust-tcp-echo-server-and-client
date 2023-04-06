use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::str;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening on: {:?}", listener.local_addr()?);
    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("Accepted connection from: {:?}", socket.peer_addr()?);
        tokio::spawn(async move {
            let mut buf = [0; 512];
            // In a loop, read data from the socket and write the data back.
            loop {
                println!("Waiting for data from socket");
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return, // stop the loop
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
                println!("Read {} bytes from socket", n);
                println!("received data: {:?}", str::from_utf8(&buf[0..n]).unwrap());
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
