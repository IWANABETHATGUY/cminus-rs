use super::op_code::OpCode;

#[derive(Debug)]
struct Function {
    pub(crate) params: usize,
    pub(crate) instructions: Vec<OpCode>,
    name: String,
}

impl Function {
    pub(crate) fn new(name: String) -> Self {
        Function {
            params: 0,
            instructions: vec![],
            name,
        }
    }
}
