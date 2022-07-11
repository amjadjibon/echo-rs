use std::io::prelude::*;
use std::net::TcpStream;
use std::env;

const SERVER_ADDR: &str = "localhost:1234";

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    println!("{:?}", args);

    let addr = args[0].clone();
    let port = args[1].clone();
    let message = args[2].clone();

    println!("{}:{}", addr, port);
    println!("{}", message);

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
