use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::path::Path;
use std::env;

fn main() {
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::from("");
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            },
            "exit" => {
                println!("Goodbye!");
                return;
            },
            command => {
                let child = Command::new(command)
                    .args(args)
                    .spawn();

                match child {
                    Ok(mut child) => {
                        let _ = child.wait();
                    },
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
        }
    }
}
