#[derive(Debug)]
pub enum OpCode {
    // the first param to store the index in constant pool
    Constant(usize),
    Return,
}

pub struct Vm {
    constant_pool: Vec<i32>,
    operations: Vec<OpCode>,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            constant_pool: vec![],
            operations: vec![],
        }
    }

    pub fn exec() {

    }
}

// TODO: need to implement constant Pool
fn disassemble_instruction(op_arr: &mut Vec<OpCode>) {
    for op in op_arr.iter() {
        match op {
            OpCode::Return => {
                println!("{:?}", op);
            }
            OpCode::Constant(_i) => {
                println!("{:?}", op);
            }
            _ => {
                unimplemented!() // TODO
            }
        }
    }
}
