use crate::parser::ast::*;

use self::{
    env::LiteralType,
    env::{Binding, Environment},
    interpreter::Evaluate,
};

pub(crate) mod env;
mod interpreter;

pub fn interpret(program: &mut Program, std_io: bool) -> Result<Environment, ()> {
    let mut env = env::Environment {
        scope_stack: vec![fxhash::FxHashMap::default()],
        call_expression_binding: Vec::new(),
        std_io,
        std_simulator: vec![],
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
    Ok(env)
}

pub(crate) fn print(binding_list: Vec<Binding>, env: &mut Environment) {
    let mut string_list = vec![];
    for binding in binding_list {
        let string = match binding {
            Binding::BooleanLiteral(val) => format!("{}", val),
            Binding::NumberLiteral(val) => format!("{}", val),
            Binding::Array(env::ArrayType::Boolean { array, .. }) => {
                format!("{:?}", array.borrow())
            }
            Binding::Array(env::ArrayType::Number { array, .. }) => format!("{:?}", array.borrow()),
            Binding::Variable(_) => {
                unimplemented!() // TODO
            }
            Binding::Void => format!("void"),
            _ => {
                panic!("not support type");
            }
        };
        string_list.push(string);
    }
    if env.std_io {
        print!("{}", string_list.join(""));
    } else {
        if let Some(last) = env.std_simulator.last_mut() {
            *last += &string_list.join("");
        } else {
            env.std_simulator.push(string_list.join(""));
        }
    }
}

pub(crate) fn println(binding_list: Vec<Binding>, env: &mut Environment) {
    let mut string_list = vec![];
    for binding in binding_list {
        let arg_string = match binding {
            Binding::NumberLiteral(val) => format!("{}", val),
            Binding::BooleanLiteral(val) => format!("{}", val),
            Binding::Array(env::ArrayType::Boolean { array, .. }) => {
                format!("{:?}", array.borrow())
            }
            Binding::Array(env::ArrayType::Number { array, .. }) => format!("{:?}", array.borrow()),
            Binding::Variable(_) => {
                unimplemented!() // TODO
            }
            Binding::Void => format!("void"),
            _ => {
                panic!("not support type");
            }
        };
        string_list.push(arg_string);
    }
    if env.std_io {
        println!("{}", string_list.join(""));
    } else {
        env.std_simulator.push(string_list.join("") + "\n");
    }
}
