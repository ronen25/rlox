use anyhow::{anyhow, Result};
use thiserror::Error;

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum OpCode {
    Return = 0,
    Constant,
    ConstantLong,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide
}

impl TryFrom<u8> for OpCode {
    type Error = ();

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            value if value == OpCode::Return as u8 => Ok(OpCode::Return),
            value if value == OpCode::Constant as u8 => Ok(OpCode::Constant),
            value if value == OpCode::ConstantLong as u8 => Ok(OpCode::ConstantLong),
            value if value == OpCode::Negate as u8 => Ok(OpCode::Negate),
            value if value == OpCode::Add as u8 => Ok(OpCode::Add),
            value if value == OpCode::Subtract as u8 => Ok(OpCode::Subtract),
            value if value == OpCode::Multiply as u8 => Ok(OpCode::Multiply),
            value if value == OpCode::Divide as u8 => Ok(OpCode::Divide),
            _ => Err(())
        }
    }
}

pub struct Chunk {
    name: String,
    code: Vec<u8>,
    constants: Vec<f32>,
    line_info: Vec<usize>,
    first_line: usize,
}

#[derive(Error, Debug)]
pub enum ChunkError {}

impl Chunk {
    const INITIAL_CAPACITY: usize = 8;

    fn new_id() -> u32 {
        static mut NEXT_ID: u32 = 0;

        // SAFETY: This is safe because nothing else ever modifies this static outside of this
        // function, and it's not multithreaded.
        let current_id = unsafe { NEXT_ID };
        unsafe {
            NEXT_ID += 1;
        }

        current_id
    }

    pub fn new(name: Option<&str>) -> Self {
        Self {
            name: name.unwrap_or(Chunk::new_id().to_string().as_str()).to_string(),
            code: Vec::with_capacity(Self::INITIAL_CAPACITY),
            constants: Vec::with_capacity(Self::INITIAL_CAPACITY),
            line_info: Vec::new(),
            first_line: 0,
        }
    }

    fn determine_line_info(&mut self, line_no: usize) {
        // If it's the first instruction pushed, initialize the line
        if self.first_line == 0 {
            self.first_line = line_no;
            self.line_info.push(1);
        } else {
            // Insert a new row number if needed
            if line_no > (self.line_info.len() - 1 + self.first_line) {
                self.line_info.push(1);
            } else {
                self.line_info[line_no - self.first_line] += 1;
            }
        }
    }

    #[inline]
    pub fn write(&mut self, byte: u8, line_no: usize) {
        self.determine_line_info(line_no);
        self.code.push(byte);
    }

    pub fn write_constant(&mut self, constant_index: u32, line_no: usize) {
        self.determine_line_info(line_no);

        let constant_bytes = constant_index.to_ne_bytes();
        for byte in constant_bytes {
            self.code.push(byte);
        }
    }

    fn get_line(&self, instr_index: usize) -> usize {
        let mut line_offset: usize = 0;

        for (line_index, line_count) in self.line_info.iter().enumerate() {
            if line_offset + *line_count <= instr_index {
                line_offset += *line_count;
            } else {
                // If adding the instruction count to this line gets us out of the instruction index,
                // we've reached our line.
                return line_index + self.first_line;
            }
        }

        0usize // TODO: Better error handling
    }

    #[cfg(debug_assertions)]
    pub fn disassemble_instruction(&self, offset: usize) -> Result<usize> {
        let instruction = self.code.get(offset).ok_or(
            anyhow!("Chunk {}: Instruction index {} out of bounds, chunk size: {}.",
            self.name, offset, self.code.len()))?;

        if let Ok(opcode) = OpCode::try_from(*instruction) {
            return match opcode {
                OpCode::Return => {
                    print!("OP_RETURN\n");

                    Ok(1)
                }
                OpCode::Constant => {
                    let constant_index = self.code.get(offset + 1).unwrap();
                    let constant = self.constants.get(*constant_index as usize).unwrap();
                    print!("OP_CONSTANT {} {}\n", constant_index, constant);

                    Ok(2)
                },
                OpCode::ConstantLong => {
                    let constant_index = self.code.get(offset + 1).unwrap();
                    let constant = self.constants.get(*constant_index as usize).unwrap();
                    print!("OP_CONSTANT_LONG {} {}\n", constant_index, constant);

                    Ok(5)
                },
                OpCode::Negate => {
                    print!("OP_NEGATE\n");
                    Ok(1)
                },
                OpCode::Add => {
                    print!("OP_ADD\n");
                    Ok(1)
                },
                OpCode::Subtract => {
                    print!("OP_SUBTRACT\n");
                    Ok(1)
                },
                OpCode::Multiply => {
                    print!("OP_MULTIPLY\n");
                    Ok(1)
                },
                OpCode::Divide => {
                    print!("OP_DIVIDE\n");
                    Ok(1)
                },
            };
        } else {
            print!("{}\n", *instruction);
        }

        Ok(1)
    }

    #[cfg(debug_assertions)]
    pub fn disassemble(&self) {
        println!("{}: ", self.name);

        let mut offset = 0;
        let mut prev_line = 0;
        while offset < self.code.len() {
            let instr_line = self.get_line(offset);

            let line_printed = if instr_line != prev_line {
                instr_line.to_string()
            } else { "|".to_string() };

            print!("{:#08x} {:>4} ", offset, line_printed);

            let instr_offset = self.disassemble_instruction(offset).unwrap();
            offset += instr_offset;
            prev_line = instr_line;
        }
    }

    pub fn add_constant(&mut self, value: f32) -> u8 {
        self.constants.push(value);
        u8::try_from(self.constants.len() - 1).unwrap() // SAFETY: UNSAFE AF. Sorry.
    }

    pub fn get_code(&self, index: usize) -> Option<&'_ u8> {
        self.code.get(index)
    }

    pub fn get_constant(&self, index: usize) -> Option<&'_ f32> {
        self.constants.get(index)
    }
}