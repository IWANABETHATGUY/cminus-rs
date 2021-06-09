use std::ops::Range;

use smol_str::SmolStr;

#[derive(Debug, PartialEq, Eq)]
pub enum OpCode {
    // the first param to store the index in constant pool
    ConstantI32(i32),
    ConstantBoolean(bool),
    Nil,
    Return,
    SubtractI32,
    MultiplyI32,
    AddI32,
    DivideI32,
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    And,
    Or,
    Neg,
    Pos,

    Pop,
    DefineGlobal(SmolStr)
}

pub fn disassemble_instruction(op: &OpCode, line_number: Range<usize>) {
    println!("op: {:?}, line: {:?}", op, line_number);
}
