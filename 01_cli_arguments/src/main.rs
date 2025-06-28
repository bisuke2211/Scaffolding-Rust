use std::env;

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <number> <string>.", args[0]);
        return;
    }

    let n: usize = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: the first argument needs to be a number.");
            return;
        }
    };

    let input_string = &args[2];
    let char_count: usize = input_string.chars().filter(|c|
        !c.is_whitespace()).count();

    println!("Number of non-space characters: {char_count}");

    for _ in 0..n {
        println!("{input_string}");
    }
}
