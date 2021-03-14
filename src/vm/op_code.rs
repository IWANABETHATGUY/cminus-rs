#[derive(Debug)]
pub enum OpCode {
    // the first param to store the index in constant pool
    ConstantI32(i32),
    Return,
    SubtractI32,
    MultiplyI32,
    AddI32,
    DivideI32,
}

pub fn disassemble_instruction(op: &OpCode, line_number: usize) {
    println!("op: {:?}, line: {}", op, line_number);
}
