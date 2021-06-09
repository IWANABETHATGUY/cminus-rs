use std::ops::Range;
use std::rc::Rc;

use super::error::Error::{self, *};
use super::op_code::disassemble_instruction;
use super::{
    op_code::OpCode::{self, *},
    value::Value,
};
use crate::expect_value;
use crate::util::variant_eq;
use fxhash::FxHashMap;
use smol_str::SmolStr;
#[derive(Debug)]
struct Compiler {
    locals: Vec<Local>,
    scope_depth: i32,
}

impl Compiler {
    fn new() -> Self {
        Self {
            scope_depth: 0,
            locals: Vec::with_capacity(256),
        }
    }

    pub fn local_count(&self) -> usize {
        self.locals.len()
    }

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.scope_depth -= 1;
    }
}

#[derive(Debug)]
struct Local {
    pub(crate) name: SmolStr,
    pub(crate) depth: i32,
}
#[derive(Debug)]
pub struct Vm {
    operations: Vec<OpCode>,
    line_number: Vec<Range<usize>>,
    stack: Vec<Value>,
    globals: FxHashMap<SmolStr, Value>,
    compiler: Compiler,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            operations: vec![],
            line_number: vec![],
            stack: Vec::with_capacity(256),
            globals: FxHashMap::default(),
            compiler: Compiler::new(),
        }
    }
    pub fn operations(&self) -> &Vec<OpCode> {
        &self.operations
    }

    pub fn exec(&mut self) -> anyhow::Result<()> {
        for (i, op) in self.operations.iter().enumerate() {
            match op {
                ConstantI32(i) => {
                    self.stack.push(Value::I32(*i));
                }
                Return => {
                    self.stack.pop();
                }
                SubtractI32 => {
                    let b = expect_value!(self);
                    let a = expect_value!(self);
                    self.stack.push(a - b);
                }
                MultiplyI32 => {
                    let b = expect_value!(self);
                    let a = expect_value!(self);
                    self.stack.push(a * b);
                }
                AddI32 => {
                    let b = expect_value!(self);
                    let a = expect_value!(self);
                    self.stack.push(a + b);
                }

                DivideI32 => {
                    let b = expect_value!(self);
                    let a = expect_value!(self);
                    self.stack.push(a / b);
                }
                ConstantBoolean(b) => {
                    self.stack.push(Value::Boolean(*b));
                }
                Equal => {
                    let b = expect_value!(self);
                    let a = expect_value!(self);
                    assert!(variant_eq(&a, &b));
                    let res = Value::Boolean(a == b);
                    self.stack.push(res);
                }
                NotEqual => {
                    let b = expect_value!(self);
                    let a = expect_value!(self);
                    assert!(variant_eq(&a, &b));
                    let res = Value::Boolean(a != b);
                    self.stack.push(res);
                }
                Greater => {
                    let b = expect_value!(self);
                    let a = expect_value!(self);
                    assert!(matches!(a, Value::I32(..)));
                    assert!(matches!(b, Value::I32(..)));
                    self.stack.push(Value::Boolean(a > b));
                }
                Less => {
                    let b = expect_value!(self);
                    let a = expect_value!(self);
                    assert!(matches!(a, Value::I32(..)));
                    assert!(matches!(b, Value::I32(..)));
                    self.stack.push(Value::Boolean(a < b));
                }
                GreaterEqual => {
                    let b = expect_value!(self);
                    let a = expect_value!(self);
                    assert!(matches!(a, Value::I32(..)));
                    assert!(matches!(b, Value::I32(..)));
                    self.stack.push(Value::Boolean(a >= b));
                }
                LessEqual => {
                    let b = expect_value!(self);
                    let a = expect_value!(self);
                    assert!(matches!(a, Value::I32(..)));
                    assert!(matches!(b, Value::I32(..)));
                    self.stack.push(Value::Boolean(a <= b));
                }

                And => {
                    let b = expect_value!(self);
                    let a = expect_value!(self);
                    if let (Value::Boolean(left), Value::Boolean(right)) = (a, b) {
                        self.stack.push(Value::Boolean(left && right));
                    } else {
                        return Err(RuntimeError(format!(
                            "error at range: {:?}, expected boolean value",
                            self.line_number[i]
                        ))
                        .into());
                    }
                }
                Or => {
                    let b = expect_value!(self);
                    let a = expect_value!(self);
                    if let (Value::Boolean(left), Value::Boolean(right)) = (a, b) {
                        self.stack.push(Value::Boolean(left || right));
                    } else {
                        return Err(RuntimeError(format!(
                            "error at range: {:?}, expected boolean value",
                            self.line_number[i]
                        ))
                        .into());
                    }
                }
                Neg => {
                    let a = expect_value!(self);
                    if let Value::I32(v) = a {
                        self.stack.push(Value::I32(-v));
                    } else {
                        return Err(RuntimeError(format!(
                            "error at range: {:?}, expected integer value, operation negative",
                            self.line_number[i]
                        ))
                        .into());
                    }
                }
                Pos => {
                    let a = expect_value!(self);
                    if let Value::I32(v) = a {
                        self.stack.push(Value::I32(v));
                    } else {
                        return Err(RuntimeError(format!(
                            "error at range: {:?}, expected integer value, operation positive",
                            self.line_number[i]
                        ))
                        .into());
                    }
                }
                Pop => {
                    self.stack.pop();
                }
                DefineGlobal(name) => {
                    let value = expect_value!(self);
                    self.globals.insert(name.clone(), value);
                }
                Nil => todo!(),
                GetGlobal(name) => {
                    if let Some(value) = self.globals.get(name) {
                        self.stack.push(value.clone());
                    } else {
                        return Err(RuntimeError(format!(
                            "error at range: {:?}, variable {} not defined",
                            self.line_number[i], name
                        ))
                        .into());
                    }
                }
                GetLocal(index) => {
                    self.stack.push(self.stack[*index].clone());
                }
                SetLocal(_) => todo!(),
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

    pub fn define_variable(&mut self, name: SmolStr, range: Range<usize>) -> anyhow::Result<()> {
        if self.compiler.scope_depth > 0 {
            self.check_if_variable_defined_in_same_scope(&name)?;
            let depth = self.compiler.scope_depth;
            let local = Local { depth, name };
            self.compiler.locals.push(local);
        } else {
            self.add_operation(DefineGlobal(name), range);
        }
        Ok(())
    }
    pub fn begin_scope(&mut self) {
        self.compiler.begin_scope();
    }

    pub fn end_scope(&mut self) {
        self.compiler.end_scope();
        let depth = self.compiler.scope_depth;
        while let Some(local) = self.compiler.locals.last() {
            if local.depth <= depth {
                break;
            }
            self.compiler.locals.pop();
            self.add_operation(Pop, 0..0);
        }
    }

    fn check_if_variable_defined_in_same_scope(&mut self, name: &SmolStr) -> anyhow::Result<()> {
        let depth = self.compiler.scope_depth;
        for local in self.compiler.locals.iter().rev() {
            if local.depth < depth {
                break;
            }
            if &local.name == name {
                return Err(
                    RuntimeError(format!("{} has already defined in this scope", name)).into(),
                );
            }
        }
        Ok(())
    }

    pub(crate) fn resolve_local(&self, name: &SmolStr) -> Option<usize> {
        self.compiler
            .locals
            .iter()
            .position(|item| &item.name == name)
    }

    pub fn scope_depth(&self) -> i32 {
        self.compiler.scope_depth
    }
}
