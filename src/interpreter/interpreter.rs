#![feature(let_chains)]
use std::{collections::HashMap, error::Error, rc::Rc};

use crate::parser::ast::*;

use super::env::{ArrayType, Binding, Environment, LiteralType};
pub trait Evaluate {
    fn evaluate(&self, env: &mut Environment) -> Result<Binding, ()>;
}

impl Evaluate for Program {
    fn evaluate(&self, env: &mut Environment) -> Result<Binding, ()> {
        println!("{}", self.declarations.len());
        for decl in self.declarations.iter() {
            match decl.evaluate(env) {
                Ok(_) => {}
                Err(_) => {
                    return Err(());
                }
            }
        }
        Ok(Binding::Void)
    }
}

impl Evaluate for Declaration {
    fn evaluate(&self, env: &mut Environment) -> Result<Binding, ()> {
        // let scope = env.scope_stack.last_mut().ok_or_else(|| {
        //     // TODO: ...
        // })?;
        match self {
            Declaration::FunctionDeclaration(func) => {
                // if scope.contains_key(&func.id.value) {
                //     // TODO: ...
                //     return Err(());
                // }

                env.define(
                    func.id.value.clone(),
                    Binding::FunctionDeclaration(Box::new(func.clone())),
                );
            }
            Declaration::VarDeclaration(var) => {
                var.evaluate(env)?;
            }
        }
        Ok(Binding::Void)
    }
}

impl Evaluate for CompoundStatement {
    fn evaluate(&self, env: &mut Environment) -> Result<Binding, ()> {
        // before every callExpression we add the binding to env.call_expression_binging, after every compoundStatement we
        // extend the params binding and clear the env.call_expression binding
        let scope = HashMap::from(env.call_expression_binding.clone());
        env.scope_stack.push(scope);
        let mut binding = Binding::Void;
        env.call_expression_binding.clear();
        for decl in self.local_declaration.iter() {
            decl.evaluate(env)?;
        }
        for stat in self.statement_list.iter() {}
        Ok(binding)
    }
}

impl Evaluate for VarDeclaration {
    fn evaluate(&self, env: &mut Environment) -> Result<Binding, ()> {
        if let Some(ref num) = self.num {
            // TODO: check the number value is usize
            let length = num.value as usize;
            match self.type_specifier.kind {
                TypeSpecifierKind::Int => {
                    env.define(
                        self.id.value.clone(),
                        Binding::Array(ArrayType::Number {
                            length,
                            array: vec![0; length],
                        }),
                    );
                }
                TypeSpecifierKind::Boolean => {
                    env.define(
                        self.id.value.clone(),
                        Binding::Array(ArrayType::Boolean {
                            length,
                            array: vec![false; length],
                        }),
                    );
                }
                _ => {
                    return Err(());
                }
            }
        } else {
            match self.type_specifier.kind {
                TypeSpecifierKind::Int => {
                    env.define(
                        self.id.value.clone(),
                        Binding::Literal(LiteralType::Number(0)),
                    );
                }
                TypeSpecifierKind::Void => {
                    env.define(self.id.value.clone(), Binding::Void);
                }
                TypeSpecifierKind::Boolean => {
                    env.define(
                        self.id.value.clone(),
                        Binding::Literal(LiteralType::Boolean(false)),
                    );
                }
            }
        }
        // scope.insert(var.id.value.clone(), Binding::Void);
        Ok(Binding::Void)
    }
}
