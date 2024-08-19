use std::env;
use std::fs;
use std::io;
use std::io::Write;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let number_of_args = args.len();

    if number_of_args < 2 {
        eprintln!("Usage: {} <file_path>", args[0].split("/").last().unwrap());
        std::process::exit(1);
    }

    let flags = parse_args(&args);
    let file_path = args.pop().unwrap();
    let contents = fs::read(&file_path).expect("Should have been able to read the file");
    let contents_text = String::from_utf8_lossy(&contents).to_string();

    let mut output = String::new();

    for item in flags {
        let msg = match item.as_str() {
            "w" => contents_text.split_whitespace().count().to_string(),
            "c" => contents.len().to_string(),
            "l" => contents_text.lines().count().to_string(),
            "m" => contents_text.chars().count().to_string(),
            _ => {
                eprintln!("Invalid flag: {}", item);
                "".to_string()
            }
        };
        output.push_str(&(msg + " "));
    }
    output.push_str(&(file_path + "\n"));
    send_output(output);
}

fn parse_args(args: &Vec<String>) -> Vec<String> {
    let default_args: Vec<String> = vec!["l".to_string(), "w".to_string(), "c".to_string()];

    if args.len() == 2 {
        return default_args;
    }

    let mut new_args: Vec<String> = Vec::new();
    for item in args {
        if item.starts_with("-") {
            for c in item.chars().skip(1) {
                new_args.push(c.to_string());
            }
        }
    }
    new_args
}

fn send_output(output: String) {
    io::stdout().write_all(output.as_bytes()).unwrap();
}
