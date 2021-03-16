use std::rc::Rc;

use super::op_code::disassemble_instruction;
use super::{op_code::OpCode, value::Value};
use crate::util::variant_eq;
use fxhash::FxHashMap;
pub struct Vm {
    operations: Vec<OpCode>,
    line_number: Vec<usize>,
    stack: Vec<Value>,
    globals: FxHashMap<String, Rc<Value>>,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            operations: vec![],
            line_number: vec![],
            stack: vec![],
            globals: FxHashMap::default()
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
                OpCode::ConstantBoolean(b) => {
                    self.stack.push(Boolean(*b));
                }
                OpCode::Equal => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    assert!(variant_eq(&a, &b));
                    let res = Boolean(a == b);
                    self.stack.push(res);
                }
                OpCode::NotEqual => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    assert!(variant_eq(&a, &b));
                    let res = Boolean(a != b);
                    self.stack.push(res);
                }
                OpCode::Greater => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    assert!(variant_eq(&a, &b));
                    let res = Boolean(a == b);
                    self.stack.push(res);
                }
                OpCode::Less => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    assert!(variant_eq(&a, &b));
                    let res = Boolean(a == b);
                    self.stack.push(res);
                }
                OpCode::GreaterEqual => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    assert!(variant_eq(&a, &b));
                    let res = Boolean(a == b);
                    self.stack.push(res);
                }
                OpCode::LessEqual => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    assert!(variant_eq(&a, &b));
                    let res = Boolean(a == b);
                    self.stack.push(res);
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

    pub fn stack(&self) -> &Vec<Value> {
        &self.stack
    }
}

