use std::cell::RefCell;
use thiserror::Error;
use crate::chunk::Chunk;
use crate::scanner::{Scanner, ScannerError, Token};

pub struct Compiler<'a> {
    scanner: RefCell<Scanner<'a>>,
}

#[derive(Error, Debug)]
pub enum CompileError {
    #[error("Failed to compile: {0}")]
    CompilationError(String),

    #[error("Scanner error")]
    ScannerError(#[from] ScannerError),
}

impl<'a, 'outlives_a: 'a> Compiler<'a> {
    pub fn new(source: &'outlives_a str) -> Self {
        Self {
            scanner: RefCell::new(Scanner::new(source))
        }
    }

    pub fn compile(&self, source: &'a str, chunk: &'a mut Chunk) -> Result<(), CompileError> {
        let mut line = 1usize;

        Ok(())
    }
}

