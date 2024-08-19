use std::env;
use std::fs;
use std::io;
use std::io::{Read, Write};

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();

    let file_path = get_file_path(&mut args);
    let flags = parse_args(&args);
    let contents = get_contents(&file_path);
    let contents_text = String::from_utf8_lossy(&contents).to_string();

    let mut output: String = flags
        .iter()
        .filter_map(|flag| get_count(flag, &contents, &contents_text))
        .map(|count| count.to_string() + " ")
        .collect();

    if let Some(file_path) = file_path {
        output.push_str(&(file_path + "\n"));
    }
    send_output(output);
}

fn get_file_path(args: &mut Vec<String>) -> Option<String> {
    if args.is_empty()
        || args
            .last()
            .expect("List of args should not be empty")
            .starts_with('-')
    {
        None
    } else {
        args.pop()
    }
}

fn parse_args(args: &Vec<String>) -> Vec<String> {
    let default_args: Vec<String> = vec!["l".to_string(), "w".to_string(), "c".to_string()];

    if args.is_empty() {
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

fn get_contents(file_path: &Option<String>) -> Vec<u8> {
    if file_path.is_none() {
        let mut buffer = Vec::new();
        io::stdin()
            .lock()
            .read_to_end(&mut buffer)
            .expect("Failed to read from stdin");
        buffer
    } else {
        fs::read(file_path.clone().unwrap()).expect("Should have been able to read the file")
    }
}

fn get_count(flag: &str, contents: &[u8], contents_text: &str) -> Option<usize> {
    let count = match flag {
        "w" => contents_text.split_whitespace().count(),
        "c" => contents.len(),
        "l" => contents_text.lines().count(),
        "m" => contents_text.chars().count(),
        _ => {
            eprintln!("Invalid flag: {}", flag);
            return None;
        }
    };
    Some(count)
}

fn send_output(output: String) {
    io::stdout().write_all(output.as_bytes()).unwrap();
}
