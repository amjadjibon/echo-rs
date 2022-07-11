use std::{thread, time::Duration};
use std::env::args;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
const ADDRESS: &str = "localhost:8080";

#[tokio::main]
async fn main() {
    // set a delay
    let delay = args().nth(1).unwrap_or_default().parse::<u64>().unwrap_or_default();

    println!("Listening on {}", ADDRESS);

    let listener = TcpListener::bind(ADDRESS).await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_connection(stream, delay).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream, delay: u64) {
    println!("Handling connection {}", stream.peer_addr().unwrap());
    let mut buffer = [0; 1024];
    let len = stream.read(&mut buffer).await.unwrap();

    let message = String::from_utf8_lossy(&buffer[..len]);
    println!("Received message: {}", message);

    // put a delay in
    thread::sleep(Duration::from_millis(delay));

    let _ = stream.write_all(message.as_bytes()).await.unwrap();
    println!("Wrote message: {}", message);
}