use smol_str::SmolStr;

use super::op_code::OpCode;

#[derive(Debug)]
pub(crate) struct Function {
    pub(crate) params: usize,
    pub(crate) instructions: usize,
    pub(crate) name: SmolStr,
}

impl Function {
    pub(crate) fn new(params: usize, name: SmolStr, instructions: usize) -> Self {
        Function {
            params,
            instructions,
            name,
        }
    }
    pub fn name(&self) -> SmolStr {
        self.name
    }
}
// typedef struct {
//   ObjFunction* function;
//   uint8_t* ip;
//   Value* slots;
// } CallFrame;

#[derive(Debug)]
pub struct CallFrame {
    pub(crate) function: Box<Function>,
    pub(crate) ip: usize,
    pub(crate) slots: usize,
}
