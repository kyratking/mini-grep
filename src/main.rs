use colored::*;
use regex::Regex;
use std::env;
use std::fs;
use std::io;
use std::process;

fn main() {
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).unwrap();

    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => {
            let filename = &args[1];
            let query = &args[2];
            let file = fs::File::open(filename).unwrap_or_else(|err| {
                eprintln!("Error opening file '{}': {}", filename, err);
                process::exit(1);
            });
            let reader = io::BufReader::new(file);
            search_on_file(reader, &query);
        }
        2 => {
            let query = &args[1];
            let stdin = io::stdin();
            let reader = stdin.lock();

            search_on_file(reader, query);
        }
        _ => {
            eprintln!("Usage:");
            eprintln!("  mini_grep <query> <file_path>    (search in a file)");
            eprintln!("  cat <file_path> | mini_grep <query> (search in piped input)");
            process::exit(1);
        }
    }
}

fn search_on_file<R: io::BufRead>(file: R, query: &String) {
    let highlighted_query = format!("{}", "$0".red());

    let mut found = false;
    let regex = Regex::new(query).unwrap();
    for line_result in file.lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(e) => {
                eprint!("Error reading line {}", e);
                process::exit(1);
            }
        };

        if regex.is_match(&line) {
            let result = regex.replace_all(&line, &highlighted_query);
            println!("{}", result);
            found = true;
        }
    }

    if !found {
        println!("No results found");
    }
}
