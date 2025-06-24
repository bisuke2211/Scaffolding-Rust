use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
        // return;
    }

    let file_path = &args[1];

    if !is_text_file(file_path) {
        eprintln!("Error: Only text files (*.txt) are allowed here.");
        std::process::exit(1);
    }

    match read_file(file_path) {
        Ok(content) => println!("{content}"),
        Err(err) => {
            eprintln!("Error reading file {err}");
            std::process::exit(1);
            // return;
        }
    }

}

fn is_text_file(file_path: &String) -> bool {
    Path::new(file_path)
        .extension()
        .map_or(false, |ext| ext == "txt")
}

fn read_file(file_path: &String) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}