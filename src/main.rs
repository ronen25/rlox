use std::{io, env};
use std::fs::File;
use std::io::{BufRead, Read, Write};
use crate::vm::VM;

mod chunk;
mod vm;
mod compiler;
mod scanner;

fn repl() -> Result<(), io::Error> {
    let stdin = io::stdin();

    print!("> ");
    _ = io::stdout().flush();
    for line in stdin.lock().lines() {
        let _line = line.unwrap();

        // Do something...

        print!("> ");
        _ = io::stdout().flush();
    }

    Ok(())
}

fn run_file(file_path: &str) -> Result<(), io::Error> {
    let mut vm = VM::new();
    let mut buffer = String::new();

    let mut source_file = File::open(file_path)?;
    _ = source_file.read_to_string(&mut buffer)?;

    Ok(())
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => repl(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: rlox [path]");
            Ok(())
        }
    }
}
