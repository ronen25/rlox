use crate::chunk::*;

mod chunk;

fn main() {
    let mut chunk = Chunk::new(None);
    chunk.write(OpCode::Return as u8, 123);

    let const_index = chunk.add_constant(1.2);
    chunk.write(OpCode::Constant as u8, 123);
    chunk.write(const_index, 123);

    let const_index = chunk.add_constant(1.2);
    chunk.write(OpCode::Constant as u8, 123);
    chunk.write(const_index, 123);

    let const_index_long = chunk.add_constant(1200.0) as u32;
    chunk.write(OpCode::ConstantLong as u8, 124);
    chunk.write_constant(const_index_long, 124);

    chunk.disassemble();
}
