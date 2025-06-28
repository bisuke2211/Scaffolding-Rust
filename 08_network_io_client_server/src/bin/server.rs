use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use std::thread;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server up on 127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Connection failed: {e}");
            }
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    println!("Received: {}", String::from_utf8_lossy(&buffer));
    stream.write(b"Message received").unwrap();
}