#![feature(let_chains)]
use std::error::Error;

use crate::parser::ast::*;

use super::env::{ArrayType, Binding, Environment, LiteralType};
pub trait Evaluate {
    fn evaluate(&mut self, env: &mut Environment) -> Result<Binding, ()>;
}

impl Evaluate for Program {
    fn evaluate(&mut self, env: &mut Environment) -> Result<Binding, ()> {
        for decl in self.declarations.iter_mut() {
            match decl.evaluate(env) {
                Ok(_) => {
                    return Ok(Binding::Void);
                }
                Err(_) => {
                    return Err(());
                }
            }
        }
        Ok(Binding::Void)
    }
}

impl Evaluate for Declaration {
    fn evaluate(&mut self, env: &mut Environment) -> Result<Binding, ()> {
        let scope = env.scope_stack.last_mut().ok_or_else(|| {
            // TODO: ...
        })?;
        match self {
            Declaration::FunctionDeclaration(func) => {
                if scope.contains_key(&func.id.value) {
                    // TODO: ...
                    return Err(());
                }
                scope.insert(
                    func.id.value.clone(),
                    Binding::FunctionDeclaration(Box::new(func.clone())),
                );
            }
            Declaration::VarDeclaration(var) => {
                if scope.contains_key(&var.id.value) {
                    // TODO: ...
                    return Err(());
                }
                // this declaration is a array
                if let Some(ref num) = var.num {
                    // TODO: check the number value is usize
                    let length = num.value as usize;
                    match var.type_specifier.kind {
                        TypeSpecifierKind::Int => {
                            scope.insert(
                                var.id.value.clone(),
                                Binding::Array(ArrayType::Number {
                                    length,
                                    array: vec![0; length],
                                }),
                            );
                        }
                        TypeSpecifierKind::Boolean => {
                            scope.insert(
                                var.id.value.clone(),
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
                    match var.type_specifier.kind {
                        TypeSpecifierKind::Int => {
                            scope.insert(
                                var.id.value.clone(),
                                Binding::Literal(LiteralType::Number(0)),
                            );
                        }
                        TypeSpecifierKind::Void => {
                            scope.insert(var.id.value.clone(), Binding::Void);
                        }
                        TypeSpecifierKind::Boolean => {
                            scope.insert(
                                var.id.value.clone(),
                                Binding::Literal(LiteralType::Boolean(false)),
                            );
                        }
                    }
                }
                scope.insert(var.id.value.clone(), Binding::Void);
            }
        }
        Ok(Binding::Void)
    }
}
