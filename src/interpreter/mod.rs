use std::collections::HashMap;

use crate::parser::ast::*;

use self::{
    env::LiteralType,
    env::{Binding, Environment},
    interpreter::Evaluate,
};

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
            Some(Binding::FunctionDeclaration(func)) => func,
            _ => {
                return Err(());
            }
        }
    } else {
        return Err(());
    };
    func.body.clone().evaluate(&mut env)?;
    Ok(Binding::Void)
}

pub(crate) fn print(binding_list: Vec<Binding>, env: &mut Environment) {
    for binding in binding_list {
        match binding {
            Binding::Literal(LiteralType::Boolean(val)) => {
                print!("{}", val);
            }
            Binding::Literal(LiteralType::Number(val)) => {
                print!("{}", val);
            }
            Binding::Array(env::ArrayType::Boolean { array, .. }) => {
                print!("{:?}", array);
            }
            Binding::Array(env::ArrayType::Number { array, .. }) => {
                print!("{:?}", array);
            }
            Binding::Variable(_) => {
                unimplemented!() // TODO
            }
            Binding::Void => {
                print!("void");
            }
            _ => {
                panic!("not support type");
            }
        }
    }
}

pub(crate) fn println(binding_list: Vec<Binding>, env: &mut Environment) {
    for binding in binding_list {
        match binding {
            Binding::Literal(LiteralType::Boolean(val)) => {
                print!("{}", val);
            }
            Binding::Literal(LiteralType::Number(val)) => {
                print!("{}", val);
            }
            Binding::Array(env::ArrayType::Boolean { array, .. }) => {
                print!("{:?}", array);
            }
            Binding::Array(env::ArrayType::Number { array, .. }) => {
                print!("{:?}", array);
            }
            Binding::Variable(_) => {
                unimplemented!() // TODO
            }
            Binding::Void => {
                print!("void");
            }
            _ => {
                panic!("not support type");
            }
        }
    }
    println!();
}