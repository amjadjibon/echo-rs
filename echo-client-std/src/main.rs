use std::io::prelude::*;
use std::net::TcpStream;

const SERVER_ADDR: &str = "localhost:1234";

fn main() {
    println!("Connecting to {}", SERVER_ADDR);
    let stream = TcpStream::connect(SERVER_ADDR);

    let mut stream = match stream {
        Ok(s) => s,
        Err(e) => {
            println!("Error connecting to {}: {}", SERVER_ADDR, e);
            return;
        }
    };

    println!("Connected to {} {}", stream.local_addr().unwrap().ip(), stream.local_addr().unwrap().port());

    println!("Sending message");
    let message = "Hello, world!";
    let _ = stream.write(message.as_bytes());
    let _ = stream.flush();
    println!("Message sent: {}", message);

    println!("Reading response");
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer);
    println!("Response: {}", String::from_utf8_lossy(&buffer[..]));
}
