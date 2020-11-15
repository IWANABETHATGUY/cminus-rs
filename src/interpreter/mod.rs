use std::collections::HashMap;

use crate::parser::ast::Program;

use self::{env::Binding, interpreter::Evaluate};

pub(crate) mod env;
mod interpreter;

pub fn interpret(program: &mut Program) -> Result<Binding, ()> {
    let mut env = env::Environment {
        scope_stack: vec![HashMap::new()],
    };
    program.evaluate(&mut env)?;
    println!("{:#?}", env);
    Ok(Binding::Void)
}
