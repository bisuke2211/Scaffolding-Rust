use std::env;
use std::net::{TcpStream};
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage {} <database address>", args[0]);
        std::process::exit(1);
    }

    let server_address= &args[1];
    println!("Attempting to connect to server at {server_address}");
    // my bad. only find out sqlite does not work like a server just now.
    // the code will stay the way it is until I can find real needs for it.

    match TcpStream::connect(server_address) {
        Ok(_) => println!("Connection successful"),
        Err(e) => {
            eprintln!("Connection error: {e}");
            std::process::exit(1);
        }
    }
    let conn_str = format!("file:{server_address}?mode=memory&cache=shared");
    let conn = Connection::open(&conn_str)?;

    let mut stmt = conn.prepare("SELECT * FROM server LIMIT 5")?;
    let rows = stmt.query_map([],
        |row| {
            Ok(row.get::<usize, String>(0)?)
        }
    )?;

    println!("First five rows:");
    for row in rows {
        println!("{}", row?);
    }

    Ok(())
}