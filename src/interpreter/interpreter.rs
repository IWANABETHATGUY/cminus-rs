use std::collections::HashMap;

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
impl Evaluate for Statement {
    fn evaluate(&self, env: &mut Environment) -> Result<Binding, ()> {
        match self {
            Statement::CompoundStatement(stat) => stat.evaluate(env),
            Statement::ExpressionStatement(stat) => {
                stat.evaluate(env)
            }
            Statement::SelectionStatement(_) => {
                unimplemented!() // TODO
            }
            Statement::IterationStatement(_) => {
                unimplemented!() // TODO
            }
            Statement::ReturnStatement(_) => {
                unimplemented!() // TODO
            }
        }
    }
}

impl Evaluate for ExpressionStatement {
    fn evaluate(&self, env: &mut Environment) -> Result<Binding, ()> {
        if let Some(ref expr) = self.expression {
            expr.evaluate(env)
        } else {
            Ok(Binding::Void)
        }
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
        for stat in self.statement_list.iter() {
            stat.evaluate(env)?;
        }
        Ok(binding)
    }
}

impl Evaluate for Expression {
    fn evaluate(&self, env: &mut Environment) -> Result<Binding, ()> {
        match self {
            Expression::Assignment(assignment) => assignment.evaluate(env),
            Expression::BinaryExpression(binary_expr) => {
                unimplemented!() // TODO
            }
            Expression::Factor(factor) => {
                factor.evaluate(env)
            }
        }
    }
}

impl Evaluate for AssignmentExpression {
    fn evaluate(&self, env: &mut Environment) -> Result<Binding, ()> {
        if let Some(box ref expr) = self.lhs.expression {
            unimplemented!() // TODO
        } else {
            let rhs_eval = self.rhs.evaluate(env)?;
            let lhs_binding = env.get_mut(&self.lhs.id.value).ok_or_else(|| {
                // TODO: ...
            })?;
            *lhs_binding = rhs_eval;
            Ok(lhs_binding.clone())
        }
    }
}

impl Evaluate for Factor {
    fn evaluate(&self, env: &mut Environment) -> Result<Binding, ()> {
        match self {
            Factor::Expression(_) => {
                unimplemented!() // TODO
            }
            Factor::Var(_) => {
                unimplemented!() // TODO
            }
            Factor::CallExpression(_) => {
                unimplemented!() // TODO
            }
            Factor::NumberLiteral(literal) => Ok(Binding::Literal(LiteralType::Number(literal.value))),
            Factor::BooleanLiteral(literal) => Ok(Binding::Literal(LiteralType::Boolean(literal.value))),
        }
    }
}
