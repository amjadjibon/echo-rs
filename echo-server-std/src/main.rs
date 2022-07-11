use std::{net::{TcpListener, TcpStream}, io::{Read, Write}, env::args};
use std::{thread, time::Duration};

const ADDRESS: &str = "localhost:8080";

fn main() {
    // set a delay
    let delay = args().nth(1).unwrap_or_default().parse::<u64>().unwrap_or_default();

    println!("Listening on {}", ADDRESS);
    let listener = TcpListener::bind(ADDRESS).unwrap();

    println!("Listening on {}", ADDRESS);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream, delay);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream, delay: u64) {
    println!("Handling connection {}", stream.peer_addr().unwrap());
    let mut buffer = [0; 1024];
    let len = stream.read(&mut buffer).unwrap();

    let message = String::from_utf8_lossy(&buffer[..len]);
    println!("Received message: {}", message);

    // put a delay in
    thread::sleep(Duration::from_millis(delay));

    let _ = stream.write_all(message.as_bytes()).unwrap();
    println!("Wrote message: {}", message);
}
