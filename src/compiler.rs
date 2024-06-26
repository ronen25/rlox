use std::cell::RefCell;
use thiserror::Error;
use crate::scanner::{Scanner, ScannerError, Token};

pub struct Compiler<'a> {
    scanner: RefCell<Scanner<'a>>,
}

#[derive(Error, Debug)]
pub enum CompileError {
    #[error("Failed to compile")]
    CompilationError,

    #[error("Scanner error")]
    ScannerError(#[from] ScannerError),
}

impl<'a, 'outlives_a: 'a> Compiler<'a> {
    pub fn new(source: &'outlives_a str) -> Self {
        Self {
            scanner: RefCell::new(Scanner::new(source))
        }
    }

    pub fn compile(&self) -> Result<(), CompileError> {
        let mut line = 1usize;

        loop {
            let token = self.scanner.borrow_mut().scan_token()?;
            let (line) = token;

            if token[0] != line {
                print!("{:>4} ", token.line);
            } else {
                print!("   | ");
            }

            print!("{:?}", token);

            if token == Token::EOF {
                break;
            }
        }

        Ok(())
    }
}

