use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {}  <dest> <source>", args[0]);
        std::process::exit(1);
    }

    let destination_path = Path::new(&args[1]);
    let source_path = Path::new(&args[2]);

    if let Some(parent_dir) = destination_path.parent() {
        fs::create_dir_all(parent_dir)?;
        println!("Created {}", parent_dir.display());
    }

    fs::rename(source_path, destination_path)?;
    println!(
        "Moved {} to {}",
        source_path.display(),
        destination_path.display()
    );

    Ok(())
}