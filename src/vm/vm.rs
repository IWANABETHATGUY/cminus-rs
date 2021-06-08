use std::ops::Range;
use std::rc::Rc;

use super::error::Error;
use super::op_code::disassemble_instruction;
use super::{
    op_code::OpCode::{self, *},
    value::Value,
};
use crate::expect_value;
use crate::util::variant_eq;
use anyhow::Result;
use fxhash::FxHashMap;
use smol_str::SmolStr;
#[derive(Debug)]
pub struct Vm {
    operations: Vec<OpCode>,
    line_number: Vec<Range<usize>>,
    stack: Vec<Value>,
    globals: FxHashMap<SmolStr, Rc<Value>>,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            operations: vec![],
            line_number: vec![],
            stack: vec![],
            globals: FxHashMap::default(),
        }
    }
    pub fn operations(&self) -> &Vec<OpCode> {
        &self.operations
    }

    pub fn exec(&mut self) -> Result<()> {
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
                    let b = expect_value!(self);
                    let a = expect_value!(self);
                    self.stack.push(a - b);
                }
                OpCode::MultiplyI32 => {
                    let b = expect_value!(self);
                    let a = expect_value!(self);
                    self.stack.push(a * b);
                }
                OpCode::AddI32 => {
                    let b = expect_value!(self);
                    let a = expect_value!(self);
                    self.stack.push(a + b);
                }

                OpCode::DivideI32 => {
                    let b = expect_value!(self);
                    let a = expect_value!(self);
                    self.stack.push(a / b);
                }
                OpCode::ConstantBoolean(b) => {
                    self.stack.push(Boolean(*b));
                }
                OpCode::Equal => {
                    let b = expect_value!(self);
                    let a = expect_value!(self);
                    assert!(variant_eq(&a, &b));
                    let res = Boolean(a == b);
                    self.stack.push(res);
                }
                OpCode::NotEqual => {
                    let b = expect_value!(self);
                    let a = expect_value!(self);
                    assert!(variant_eq(&a, &b));
                    let res = Boolean(a != b);
                    self.stack.push(res);
                }
                OpCode::Greater => {
                    let b = expect_value!(self);
                    let a = expect_value!(self);
                    assert!(variant_eq(&a, &b));
                    let res = Boolean(a == b);
                    self.stack.push(res);
                }
                OpCode::Less => {
                    let b = expect_value!(self);
                    let a = expect_value!(self);
                    assert!(variant_eq(&a, &b));
                    let res = Boolean(a == b);
                    self.stack.push(res);
                }
                OpCode::GreaterEqual => {
                    let b = expect_value!(self);
                    let a = expect_value!(self);
                    assert!(variant_eq(&a, &b));
                    let res = Boolean(a == b);
                    self.stack.push(res);
                }
                OpCode::LessEqual => {
                    let b = expect_value!(self);
                    let a = expect_value!(self);
                    assert!(variant_eq(&a, &b));
                    let res = Boolean(a == b);
                    self.stack.push(res);
                }
                OpCode::Pop => {
                    self.stack.pop();
                },
                OpCode::DefineGlobal(name) => {
                    let value = expect_value!(self);
                    self.globals.insert(name.clone(), Rc::new(value));
                }
                OpCode::Nil => todo!(),
            }
            // DEBUG: start
            if cfg!(debug_assertions) {
                disassemble_instruction(&op, self.line_number[i].clone());
                println!("stack: {:?}", self.stack);
            }
            // DEBUG: end
        }
        Ok(())
    }

    pub fn add_operation(&mut self, op: OpCode, line_number: Range<usize>) {
        self.operations.push(op);
        self.line_number.push(line_number);
    }

    pub fn stack(&self) -> &Vec<Value> {
        &self.stack
    }

    pub fn define_variable(&mut self, name: SmolStr, range: Range<usize>) {
        self.add_operation(DefineGlobal(name), range);
    }
}
