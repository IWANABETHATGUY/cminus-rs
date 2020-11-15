use std::collections::HashMap;

use crate::parser::ast::*;

use self::{env::Binding, interpreter::Evaluate};

pub(crate) mod env;
mod interpreter;

pub fn interpret(program: &mut Program) -> Result<Binding, ()> {
    let mut env = env::Environment {
        scope_stack: vec![HashMap::new()],
        call_expression_binding: HashMap::new(),
    };
    program.evaluate(&mut env)?;
    let func = if let Some(scope) = env.scope_stack.last() {
        match scope.get("main") {
            Some(Binding::FunctionDeclaration(box func)) => func,
            _ => {
                return Err(());
            }
        }
    } else {
        return Err(());
    };
    func.body.clone().evaluate(&mut env)?;
    println!("{:#?}", env.scope_stack);
    Ok(Binding::Void)
}
