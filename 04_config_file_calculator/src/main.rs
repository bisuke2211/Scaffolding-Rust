use chrono::Local;
use std::fs;
use std::io::{self, Write};
use toml::Value;

#[derive(Debug)]
struct Config {
    show_datetime: bool,
}

impl Config {
    fn new() -> Self {
        Config {
            show_datetime: false,
        }
    }

    fn from_file(file_path: &str) -> Self {
        let contents = fs::read_to_string(file_path);
        match contents {
            Ok(content) => {
                let parsed: Result<Value, _> = content.parse::<Value>();
                if let Ok(value) = parsed {
                    if let Some(show_datetime) = value.get("show_datetime").and_then(Value::as_bool)
                    {
                        return Config {show_datetime}
                    }
                }
                eprintln!("Warning: Failed to parse config file.");
            }
            Err(_) => eprintln!("Warning: Config file not found. Default used."),
        }
        Config::new()
    }
}

fn main() {
    let config = Config::from_file("settings.toml");

    println!("Welcome to the Rust Calculator");
    println!("Enter calculations in this format: <operand1> <operand2> <operator>");
    println!("Supported operators: + - * /");
    println!("Type 'exit' to exit the program anytime\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        if config.show_datetime {
            let now = Local::now();
            println!("Current date and time: {}", now.format("%Y-%m-%d %H:%M"));
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 3 {
            println!("Error: Invalid input format.");
            continue;
        }

        let operand1: f64 = match parts[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: Invalid number {}", parts[0]);
                continue;
            }
        };

        let operand2: f64 = match parts[1].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error: Invalid number {}", parts[1]);
                continue;
            }
        };
        
        let operator = parts[2];
        let result = match operator {
            "+" => Some(operand1 + operand2),
            "-" => Some(operand1 - operand2),
            "*" => Some(operand1 * operand2),
            "/" => {
                if operand2 == 0.0 {
                    println!{"Error: Division by zeor"};
                    None
                } else {
                    Some(operand1 / operand2)
                }
            }
            _ => {
                println!("Error: Unsupported operator {}", operator);
                None
            }
        };

        if let Some(value) = result {
            println!("Result: {}", value);
        }
    }
}