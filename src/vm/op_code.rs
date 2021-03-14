use super::value::Value;
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

pub struct Vm {
    constant_pool: Vec<f64>,
    operations: Vec<OpCode>,
    line_number: Vec<usize>,
    stack: Vec<Value>,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            constant_pool: vec![],
            operations: vec![],
            line_number: vec![],
            stack: vec![],
        }
    }

    pub fn exec(&mut self) {
        use Value::*;
        for (i, op) in self.operations.iter().enumerate() {
            match op {
                OpCode::ConstantI32(i) => {
                    self.stack.push(I32(*i));
                }
                OpCode::Return => {
                    self.stack.pop();
                }
                OpCode::SubtractI32 => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a - b);
                }
                OpCode::MultiplyI32 => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a * b);
                }
                OpCode::AddI32 => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                }

                OpCode::DivideI32 => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a / b);
                }
            }
            // DEBUG: start
            if cfg!(debug_assertions) {
                disassemble_instruction(&op, self.line_number[i]);
                println!("stack: {:?}", self.stack);
            }
            // DEBUG: end
        }
    }

    pub fn add_operation(&mut self, op: OpCode, line_number: usize) {
        self.operations.push(op);
        self.line_number.push(line_number);
    }
}

fn disassemble_instruction(op: &OpCode, line_number: usize) {
    println!("op: {:?}, line: {}", op, line_number);
}
