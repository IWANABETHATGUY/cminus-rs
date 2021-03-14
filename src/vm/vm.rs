use super::op_code::disassemble_instruction;
use super::{op_code::OpCode, value::Value};
use crate::util::variant_eq;

pub struct Vm {
    operations: Vec<OpCode>,
    line_number: Vec<usize>,
    stack: Vec<Value>,
}

impl Vm {
    pub fn new() -> Self {
        Self {
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

#[cfg(test)]
mod test_vm {
    use super::*;

    #[test]
    fn test_calculation() {
        // subtract
        let mut vm = Vm::new();
        vm.add_operation(OpCode::ConstantI32(20), 0);
        vm.add_operation(OpCode::ConstantI32(10), 0);
        vm.add_operation(OpCode::SubtractI32, 0);
        vm.exec();
        let result = vec![Value::I32(10)];
        assert_eq!(vm.stack(), &result);
        // add
        let mut vm = Vm::new();
        vm.add_operation(OpCode::ConstantI32(20), 0);
        vm.add_operation(OpCode::ConstantI32(10), 0);
        vm.add_operation(OpCode::AddI32, 0);
        vm.exec();
        let result = vec![Value::I32(30)];
        assert_eq!(vm.stack(), &result);
        // divide
        let mut vm = Vm::new();
        vm.add_operation(OpCode::ConstantI32(20), 0);
        vm.add_operation(OpCode::ConstantI32(10), 0);
        vm.add_operation(OpCode::DivideI32, 0);
        vm.exec();
        let result = vec![Value::I32(2)];
        assert_eq!(vm.stack(), &result);
        // multiply
        let mut vm = Vm::new();
        vm.add_operation(OpCode::ConstantI32(20), 0);
        vm.add_operation(OpCode::ConstantI32(10), 0);
        vm.add_operation(OpCode::MultiplyI32, 0);
        vm.exec();
        let result = vec![Value::I32(200)];
        assert_eq!(vm.stack(), &result);
        // expression
        let mut vm = Vm::new();
        vm.add_operation(OpCode::ConstantI32(2), 0);
        vm.add_operation(OpCode::ConstantI32(4), 0);
        vm.add_operation(OpCode::AddI32, 0);
        vm.add_operation(OpCode::ConstantI32(3), 0);
        vm.add_operation(OpCode::DivideI32, 0);
        vm.exec();
        assert_eq!(vm.stack(), &vec![Value::I32(2)]);
    }
}
