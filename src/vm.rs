use thiserror::Error;
use crate::chunk::{Chunk, OpCode};
use crate::compiler::{CompileError, Compiler};

pub struct VM<'a> {
    compiler: Compiler<'a>,
    ip: usize,
    stack: Vec<f32>
}

enum BinaryOp {
    Addition, Subtraction, Multiplication, Division
}

#[derive(Error, Debug)]
pub enum InterpretError {
    #[error("Compile error: {0}")]
    CompileError(#[from] CompileError),

    #[error("Runtime error")]
    RuntimeError
}

macro_rules! binary_op {
    ($stack:expr, $op:tt) => {
        let a = $stack.pop().unwrap();
        let b = $stack.pop().unwrap();

        $stack.push(a $op b);
    };
}

impl<'a, 'outlives_a: 'a> VM<'a> {
    pub fn new(source: &'outlives_a str) -> Self {
        const STACK_SIZE: usize = 256;

        Self {
            compiler: Compiler::new(source),
            ip: 0,
            stack: Vec::with_capacity(STACK_SIZE)
        }
    }

    pub fn interpret(&mut self, source: &str) -> Result<(), InterpretError> {
        let mut chunk = Chunk::new(None);
        self.compiler.compile(source, &mut chunk)?;

        loop {
            let instruction_byte = chunk.get_code(self.ip).unwrap();
            if let Ok(instruction) = OpCode::try_from(*instruction_byte) {
                #[cfg(debug_assertions)]
                {
                    print!("[ ");
                    for value in self.stack.iter() {
                        print!("{}, ", value);
                    }
                    print!("] ");

                    chunk.disassemble_instruction(self.ip).unwrap();
                }

                self.ip += 1;

                match instruction {
                    OpCode::Return => {
                        let stack_top = self.stack.pop().unwrap_or(0f32);
                        println!("{}", stack_top);

                        return Ok(());
                    },
                    OpCode::Constant => {
                        let constant_index = *chunk.get_code(self.ip).unwrap();
                        let constant_value = *chunk.get_constant(constant_index as usize).unwrap();

                        self.stack.push(constant_value);

                        // OP_CONST is two bytes long
                        self.ip += 1;
                    }
                    OpCode::Negate => {
                        let stack_top_mut = self.stack.last_mut().unwrap();
                        *stack_top_mut *= -1f32;
                    }
                    OpCode::Add => {
                        binary_op!((&mut self.stack), +);
                    },
                    OpCode::Subtract => {
                        binary_op!((&mut self.stack), -);
                    },
                    OpCode::Multiply => {
                        binary_op!((&mut self.stack), *);
                    },
                    OpCode::Divide => {
                        binary_op!((&mut self.stack), /);
                    },
                    _ => {
                        let compile_err_msg = format!("Unknown instruction byte {}", instruction_byte);
                        return Err(InterpretError::CompileError(CompileError::CompilationError(compile_err_msg)));
                    }
                }
            } else {
                let compile_err_msg = format!("Unknown instruction byte {}", instruction_byte);
                return Err(InterpretError::CompileError(CompileError::CompilationError(compile_err_msg)));
            }
        }
    }
}