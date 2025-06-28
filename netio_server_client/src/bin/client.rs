use std::net::TcpStream;
use std::io::{Read, Write};
use std::env;

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage {} <text to be sent to the server>", args[0]);
        std::process::exit(1);
    }

    let message: &String = &args[1];
    
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    stream.write(message.as_bytes())?;

    let mut buffer = [0; 512];
    let n = stream.read(&mut buffer)?;
    println!("Server response: {}", 
        String::from_utf8_lossy(&buffer[..n])
    );

    Ok(())
}