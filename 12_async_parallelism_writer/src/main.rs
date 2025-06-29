use std::env;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::sync::Arc;
use rayon::prelude::*;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args: Vec<String>  = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage {} <number of copies you need>", args[0]);
        std::process::exit(1);
    }

    let n: usize = args[1]
        .parse()
        .expect("Your argument should be a number");
    
    let file_names: Vec<String> = (1..=n)
        .map(|i| format!("output{}", i)).collect();
    let file_names_arc = Arc::new(file_names);

    let (tx, mut rx) = mpsc::channel::<String>(100);

    let input_task = tokio::spawn(async move {
        let stdin = io::stdin();
        let mut buffer = String::new();

        println!("Enter your thoughts or exit (Ctrl D):");
        while stdin.read_line(&mut buffer).is_ok() {
            let input = buffer.trim().to_string();
            if input.is_empty() {
                break;
            }

            if tx.send(input).await.is_err() {
                eprintln!("Failed to send content to backend. Exiting...");
                break;
            }

            buffer.clear();

            tokio::task::yield_now().await; // without it the entire input part will be blocking
        }
    });

    let file_names = Arc::clone(&file_names_arc);
    let writer_task = tokio::spawn(async move {
        while let Some(input) = rx.recv().await {
            file_names.par_iter().for_each(|file_name| {
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(file_name)
                    .expect("Failed to open files");
                writeln!(file, "{}", input).expect("Failed to write to file");
                println!("Writing to file: {}", file_name);
                // file.flush().expect("Failed to flush content to files...") not necessary
            });
            println!("Wrote to files {input}");
        }
    });

    let _ = tokio::join!(input_task, writer_task);
    println!("All content now written. Program exiting...");

    Ok(())

}

// important message by GPT
// https://poe.com/s/vndmyMIfW2lDbsJUPVlj