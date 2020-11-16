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
                    Binding::FunctionDeclaration(std::rc::Rc::new(func.clone())),
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
// TODO: check the nested returnStatement return binding
impl Statement {
    fn evaluate(&self, env: &mut Environment) -> Result<Option<Binding>, ()> {
        match self {
            Statement::CompoundStatement(stmt) => stmt.evaluate(env),
            Statement::ExpressionStatement(stmt) => {
                if let Err(_) = stmt.evaluate(env) {
                    Err(())
                } else {
                    Ok(None)
                }
            }
            Statement::SelectionStatement(_) => {
                unimplemented!() // TODO
            }
            Statement::IterationStatement(_) => {
                unimplemented!() // TODO
            }
            Statement::ReturnStatement(stmt) => {
                if let Ok(binding) = stmt.evaluate(env) {
                    Ok(Some(binding))
                } else {
                    Err(())
                }
            }
        }
    }
}

impl Evaluate for ExpressionStatement {
    fn evaluate(&self, env: &mut Environment) -> Result<Binding, ()> {
        if let Some(ref expr) = self.expression {
            if let Ok(binding) = expr.evaluate(env) {
                return Ok(binding);
            } else {
                return Err(());
            }
        } else {
            return Ok(Binding::Void);
        }
    }
}
impl Evaluate for ReturnStatement {
    fn evaluate(&self, env: &mut Environment) -> Result<Binding, ()> {
        if let Some(expr) = &self.expression {
            expr.evaluate(env)
        } else {
            Ok(Binding::Void)
        }
    }
}
impl CompoundStatement {
    pub fn evaluate(&self, env: &mut Environment) -> Result<Option<Binding>, ()> {
        // before every callExpression we add the binding to env.call_expression_binging, after every compoundStatement we
        // extend the params binding and clear the env.call_expression binding
        let scope = HashMap::from(env.call_expression_binding.clone());
        env.scope_stack.push(scope);
        env.call_expression_binding.clear();
        for decl in self.local_declaration.iter() {
            if let Err(_) = decl.evaluate(env) {
                return Err(());
            }
            // decl.evaluate(env)?;
        }
        for stat in self.statement_list.iter() {
            match stat.evaluate(env) {
                Ok(binding) => {
                    println!("{:?}", env.scope_stack.last());
                    env.scope_stack.pop();
                    return Ok(binding);
                }
                _ => {
                    env.scope_stack.pop();
                    return Err(());
                }
            }
        }
        env.scope_stack.pop();
        Ok(None)
    }
}

impl Evaluate for Expression {
    fn evaluate(&self, env: &mut Environment) -> Result<Binding, ()> {
        match self {
            Expression::Assignment(assignment) => assignment.evaluate(env),
            Expression::BinaryExpression(binary_expr) => binary_expr.evaluate(env),
            Expression::Factor(factor) => factor.evaluate(env),
        }
    }
}

impl Evaluate for AssignmentExpression {
    fn evaluate(&self, env: &mut Environment) -> Result<Binding, ()> {
        if let Some(box ref expr) = self.lhs.expression {
            // this is a array expression assignment
            unimplemented!() // TODO
        } else {
            let rhs_eval = self.rhs.evaluate(env)?;
            let lhs_binding = env.get_mut(&self.lhs.id.value).ok_or_else(|| {
                println!(
                    "the variable {:?} can't be found in this module",
                    self.lhs.id.value
                );
            })?;
            *lhs_binding = rhs_eval;
            Ok(lhs_binding.clone())
        }
    }
}

impl Evaluate for BinaryExpression {
    fn evaluate(&self, env: &mut Environment) -> Result<Binding, ()> {
        let left_eval = &self.left.evaluate(env)?;
        let right_eval = &self.right.evaluate(env)?;
        match (left_eval, right_eval) {
            (
                Binding::Literal(LiteralType::Number(_)),
                Binding::Literal(LiteralType::Number(_)),
            ) => evaluate_binary_expression_literal(left_eval, right_eval, &self.operation),
            (
                Binding::Literal(LiteralType::Boolean(_)),
                Binding::Literal(LiteralType::Boolean(_)),
            ) => evaluate_binary_expression_literal(left_eval, right_eval, &self.operation),
            (Binding::Literal(_), Binding::Variable(var)) => {
                if let Some(right_var) = env.get(var) {
                    evaluate_binary_expression_literal(left_eval, right_var, &self.operation)
                } else {
                    Err(())
                }
            }
            (Binding::Variable(var), Binding::Literal(_)) => {
                if let Some(left_var) = env.get(var) {
                    evaluate_binary_expression_literal(left_var, right_eval, &self.operation)
                } else {
                    Err(())
                }
            }
            _ => Err(()),
        }
    }
}

fn evaluate_binary_expression_literal(
    m: &Binding,
    n: &Binding,
    op: &Operation,
) -> Result<Binding, ()> {
    match (m, n) {
        (Binding::Literal(LiteralType::Number(a)), Binding::Literal(LiteralType::Number(b))) => {
            match op {
                Operation::GT => Ok(Binding::Literal(LiteralType::Boolean(a > b))),
                Operation::LT => Ok(Binding::Literal(LiteralType::Boolean(a < b))),
                Operation::GE => Ok(Binding::Literal(LiteralType::Boolean(a >= b))),
                Operation::LE => Ok(Binding::Literal(LiteralType::Boolean(a <= b))),
                Operation::EQ => Ok(Binding::Literal(LiteralType::Boolean(a > b))),
                Operation::NE => Ok(Binding::Literal(LiteralType::Boolean(a == b))),
                Operation::PLUS => Ok(Binding::Literal(LiteralType::Number(a + b))),
                Operation::MINUS => Ok(Binding::Literal(LiteralType::Number(a - b))),
                Operation::MULTIPLY => Ok(Binding::Literal(LiteralType::Number(a * b))),
                Operation::DIVIDE => Ok(Binding::Literal(LiteralType::Number(a / b))),
            }
        }
        (Binding::Literal(LiteralType::Boolean(a)), Binding::Literal(LiteralType::Boolean(b))) => {
            match op {
                Operation::GT => Ok(Binding::Literal(LiteralType::Boolean(a > b))),
                Operation::LT => Ok(Binding::Literal(LiteralType::Boolean(a < b))),
                Operation::GE => Ok(Binding::Literal(LiteralType::Boolean(a >= b))),
                Operation::LE => Ok(Binding::Literal(LiteralType::Boolean(a <= b))),
                Operation::EQ => Ok(Binding::Literal(LiteralType::Boolean(a > b))),
                Operation::NE => Ok(Binding::Literal(LiteralType::Boolean(a == b))),
                _ => Err(()),
            }
        }
        _ => Err(()),
    }
}

impl Evaluate for Factor {
    fn evaluate(&self, env: &mut Environment) -> Result<Binding, ()> {
        match self {
            Factor::Expression(expr) => expr.evaluate(env),
            Factor::Var(var) => env
                .get(&var.id.value)
                .and_then(|bind| Some(bind.clone()))
                .ok_or_else(|| {}),
            Factor::CallExpression(call_expr) => call_expr.evaluate(env),
            Factor::NumberLiteral(literal) => {
                Ok(Binding::Literal(LiteralType::Number(literal.value)))
            }
            Factor::BooleanLiteral(literal) => {
                Ok(Binding::Literal(LiteralType::Boolean(literal.value)))
            }
        }
    }
}

impl Evaluate for CallExpression {
    fn evaluate(&self, env: &mut Environment) -> Result<Binding, ()> {
        let func_name = &self.id.value;
        let func_decl = env.get(func_name).ok_or_else(|| {})?;
        if let Binding::FunctionDeclaration(decl) = func_decl {
            let decl = decl.clone();
            let call_expression_scope =
                prepare_call_expression_binding(env, &decl.params.clone(), &self.arguments)?;
            env.call_expression_binding = call_expression_scope;
            match decl.body.evaluate(env) {
                Ok(Some(binding)) => {
                    // env.scope_stack.pop();
                    return Ok(binding);
                }
                Err(_) => {
                    // env.scope_stack.pop();
                    return Err(());
                }
                _ => {
                    // env.scope_stack.pop();
                    return Ok(Binding::Void);
                }
            }
        } else {
            return Err(());
        }
    }
}

fn prepare_call_expression_binding(
    env: &mut Environment,
    params: &Params,
    arguments: &Vec<Expression>,
) -> Result<HashMap<String, Binding>, ()> {
    match params {
        Params::Void => {
            if arguments.len() != 0 {
                return Err(());
            }
            return Ok(HashMap::new());
        }
        Params::ParamsList { params } => {
            if params.len() != arguments.len() {
                return Err(());
            }
            let mut map = HashMap::new();
            for (param, arg) in params.iter().zip(arguments.iter()) {
                let binding = generate_assignable_binding(env, param, arg)?;
                map.insert(param.id.value.clone(), binding);
            }
            Ok(map)
        }
    }
}

fn generate_assignable_binding(
    env: &mut Environment,
    param: &Parameter,
    arg: &Expression,
) -> Result<Binding, ()> {
    if param.is_array {
        unimplemented!() // TODO
    } else {
        let arg_binding = arg.evaluate(env)?;
        match (&param.type_specifier.kind, &arg_binding) {
            (TypeSpecifierKind::Void, Binding::Void) => Ok(arg_binding),
            (TypeSpecifierKind::Boolean, Binding::Literal(LiteralType::Boolean(_))) => {
                Ok(arg_binding)
            }
            (TypeSpecifierKind::Int, Binding::Literal(LiteralType::Number(_))) => Ok(arg_binding),
            _ => Err(()),
        }
    }
}
