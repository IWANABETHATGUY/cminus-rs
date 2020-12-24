use std::{cell::RefCell, rc::Rc};

use super::env::{ArrayType, Binding, Environment, IntoLiteral, LiteralType};
use crate::parser::ast::*;
use fxhash::FxHashMap;

pub trait Evaluate {
    fn evaluate(&self, env: &mut Environment) -> Result<Binding, ()>;
}

impl Evaluate for Program {
    fn evaluate(&self, env: &mut Environment) -> Result<Binding, ()> {
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
        match self {
            Declaration::FunctionDeclaration(func) => {
                // match func.id.value.as_ref() {
                //     id @ "print"| id @"println"  => {
                //         eprintln!("`{}` is built-in function, can't be redefined", id);
                //         return Err(());
                //     }
                //     _ => {}
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

impl VarDeclaration {
    fn evaluate(&self, env: &mut Environment) -> Result<Binding, ()> {
        // let local_scope = env.scope_stack.last_mut().unwrap();
        if env
            .scope_stack
            .last_mut()
            .unwrap()
            .contains_key(&self.id.value)
        {
            println!("Can not declare a variable twice");
            return Err(());
        } else {
            // TODO: need type checking here
            match self.num {
                None => match self.type_specifier.kind {
                    TypeSpecifierKind::Int => {
                        let init = if let Some(ref initializer) = self.initializer {
                            initializer.evaluate(env)?
                        } else {
                            Binding::NumberLiteral(0)
                        };
                        env.scope_stack
                            .last_mut()
                            .unwrap()
                            .insert(self.id.value.clone(), init);
                    }
                    TypeSpecifierKind::Boolean => {
                        let init = if let Some(ref initializer) = self.initializer {
                            initializer.evaluate(env)?
                        } else {
                            Binding::BooleanLiteral(false)
                        };
                        env.scope_stack
                            .last_mut()
                            .unwrap()
                            .insert(self.id.value.clone(), init);
                    }
                    TypeSpecifierKind::Void => {
                        let init = if let Some(ref initializer) = self.initializer {
                            initializer.evaluate(env)?
                        } else {
                            Binding::Void
                        };
                        env.scope_stack
                            .last_mut()
                            .unwrap()
                            .insert(self.id.value.clone(), init);
                    }
                },
                Some(ref num) => {
                    // TODO: here also need to handle with declaration with initializer
                    let length = num.value as usize;

                    match self.type_specifier.kind {
                        TypeSpecifierKind::Int => {
                            let initialized_array = self
                                .get_initialized_array(env, &self.type_specifier.kind, length)?
                                .into_iter()
                                .map(|item| match item {
                                    LiteralType::Boolean(n) => {
                                        unreachable!();
                                    }
                                    LiteralType::Number(n) => n,
                                })
                                .collect::<Vec<_>>();
                            env.scope_stack.last_mut().unwrap().insert(
                                self.id.value.clone(),
                                Binding::Array(ArrayType::Number {
                                    length,
                                    array: Rc::new(RefCell::new(initialized_array)),
                                }),
                            );
                        }
                        TypeSpecifierKind::Boolean => {
                            let initialized_array = self
                                .get_initialized_array(env, &self.type_specifier.kind, length)?
                                .into_iter()
                                .map(|item| match item {
                                    LiteralType::Boolean(n) => n,
                                    LiteralType::Number(n) => {
                                        unreachable!();
                                    }
                                })
                                .collect::<Vec<_>>();
                            env.scope_stack.last_mut().unwrap().insert(
                                self.id.value.clone(),
                                Binding::Array(ArrayType::Boolean {
                                    length,
                                    array: Rc::new(RefCell::new(initialized_array)),
                                }),
                            );
                        }
                        _ => {
                            return Err(());
                        }
                    }
                }
            }
        }

        Ok(Binding::Void)
    }
    fn get_initialized_array(
        &self,
        env: &mut Environment,
        typ: &TypeSpecifierKind,
        length: usize,
    ) -> Result<Vec<LiteralType>, ()> {
        if let Some(ref init) = self.array_initializer {
            // TODO: if the initializer length is greater than the length , should warn
            let mut vec = vec![0.get_literal(); length];
            let len = length.min(init.len());
            match typ {
                TypeSpecifierKind::Int => {
                    for i in 0..len {
                        vec[i] = init[i].evaluate(env)?.get_literal();
                    }
                    Ok(vec)
                }
                TypeSpecifierKind::Boolean => {
                    return Err(());
                }
                _ => {
                    return Err(());
                }
            }
        } else {
            match typ {
                TypeSpecifierKind::Int => Ok(vec![0.get_literal(); length]),
                TypeSpecifierKind::Boolean => Ok(vec![false.get_literal(); length]),
                _ => {
                    println!("the array only support int and boolean now!");
                    Err(())
                }
            }
        }
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
            Statement::SelectionStatement(stmt) => {
                if let Ok(binding) = stmt.test.evaluate(env) {
                    match binding {
                        Binding::BooleanLiteral(value) => {
                            if value {
                                return stmt.consequent.evaluate(env);
                            } else if let Some(alternative) = &stmt.alternative {
                                return alternative.evaluate(env);
                            } else {
                                return Ok(None);
                            }
                        }
                        _ => {
                            println!("test expression should be an boolean expression");
                            return Err(());
                        }
                    }
                } else {
                    return Err(());
                };
            }
            Statement::IterationStatement(stmt) => {
                while let Ok(binding) = stmt.test.evaluate(env) {
                    match binding {
                        Binding::BooleanLiteral(value) => {
                            if value {
                                match stmt.body.evaluate(env) {
                                    Ok(option_binding) => {
                                        if let Some(binding) = option_binding {
                                            return Ok(Some(binding));
                                        }
                                    }
                                    Err(_) => {
                                        return Err(());
                                    }
                                }
                            } else {
                                break;
                            }
                        }
                        _ => {
                            println!("while test expression should be an boolean expression");
                            return Err(());
                        }
                    }
                }
                Ok(None)
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
        let mut scope = {
            let mut map = FxHashMap::default();
            while let Some((string, binding)) = env.call_expression_binding.pop() {
                map.insert(string, binding);
            }
            map
        };
        env.scope_stack.push(scope);
        for decl in self.local_declaration.iter() {
            if let Err(_) = decl.evaluate(env) {
                return Err(());
            }
        }
        let mut option_binding = None;
        for stat in self.statement_list.iter() {
            match stat.evaluate(env) {
                Ok(None) => {}
                Ok(Some(binding)) => {
                    option_binding = Some(binding);
                    break;
                }
                _ => {
                    env.scope_stack.pop();
                    return Err(());
                }
            }
        }
        // println!("{}:{:?}", env.scope_stack.len(), env.scope_stack.last());
        env.scope_stack.pop();
        Ok(option_binding)
    }
}

impl Evaluate for Expression {
    fn evaluate(&self, env: &mut Environment) -> Result<Binding, ()> {
        match self {
            Expression::BinaryExpression(binary_expr) => binary_expr.evaluate(env),
            Expression::Factor(factor) => factor.evaluate(env),
            Expression::Assignment(assignment) => assignment.evaluate(env),
        }
    }
}

impl Evaluate for AssignmentExpression {
    fn evaluate(&self, env: &mut Environment) -> Result<Binding, ()> {
        if let Some(box ref expr) = self.lhs.expression {
            let index_eval = expr.evaluate(env)?;
            let rhs_eval = self.rhs.evaluate(env)?;
            if let Binding::NumberLiteral(index) = index_eval {
                match env.get_mut(&self.lhs.id.value).ok_or_else(|| {
                    println!(
                        "the variable {:?} can't be found in this scope",
                        self.lhs.id.value
                    );
                })? {
                    Binding::Array(arr) => match arr {
                        ArrayType::Boolean { array, .. }
                            if matches!(rhs_eval, Binding::BooleanLiteral(_)) =>
                        {
                            array.borrow_mut()[index as usize] =
                                rhs_eval.clone().into_boolean_literal().unwrap();
                            Ok(rhs_eval)
                        }
                        ArrayType::Number { array, .. }
                            if matches!(rhs_eval, Binding::NumberLiteral(_)) =>
                        {
                            array.borrow_mut()[index as usize] =
                                rhs_eval.clone().into_number_literal().unwrap();
                            Ok(rhs_eval)
                        }
                        _ => Err(()),
                    },
                    _ => {
                        println!(
                            "the variable {:?} should be found in this scope",
                            self.lhs.id.value
                        );
                        Err(())
                    }
                }
            } else {
                unimplemented!() // TODO
            }
        // env.get_mut();
        // this is a array expression assignment
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
            (Binding::NumberLiteral(_), Binding::NumberLiteral(_)) => {
                evaluate_binary_expression_literal(left_eval, right_eval, &self.operation)
            }
            (Binding::BooleanLiteral(_), Binding::BooleanLiteral(_)) => {
                evaluate_binary_expression_literal(left_eval, right_eval, &self.operation)
            }
            (
                left_eval @ Binding::NumberLiteral(_) | left_eval @ Binding::BooleanLiteral(_),
                Binding::Variable(var),
            ) => {
                if let Some(right_var) = env.get(var) {
                    evaluate_binary_expression_literal(left_eval, right_var, &self.operation)
                } else {
                    Err(())
                }
            }
            (
                Binding::Variable(var),
                right_eval @ Binding::NumberLiteral(_) | right_eval @ Binding::BooleanLiteral(_),
            ) => {
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
#[inline]
fn evaluate_binary_expression_literal(
    m: &Binding,
    n: &Binding,
    op: &Operation,
) -> Result<Binding, ()> {
    match (m, n) {
        (Binding::NumberLiteral(a), Binding::NumberLiteral(b)) => match op {
            Operation::GT => Ok(Binding::BooleanLiteral(a > b)),
            Operation::LT => Ok(Binding::BooleanLiteral(a < b)),
            Operation::GE => Ok(Binding::BooleanLiteral(a >= b)),
            Operation::LE => Ok(Binding::BooleanLiteral(a <= b)),
            Operation::EQ => Ok(Binding::BooleanLiteral(a == b)),
            Operation::NE => Ok(Binding::BooleanLiteral(a != b)),
            Operation::PLUS => Ok(Binding::NumberLiteral(a + b)),
            Operation::MINUS => Ok(Binding::NumberLiteral(a - b)),
            Operation::MULTIPLY => Ok(Binding::NumberLiteral(a * b)),
            Operation::DIVIDE => Ok(Binding::NumberLiteral(a / b)),
        },
        (Binding::BooleanLiteral(a), Binding::BooleanLiteral(b)) => match op {
            Operation::GT => Ok(Binding::BooleanLiteral(a > b)),
            Operation::LT => Ok(Binding::BooleanLiteral(a < b)),
            Operation::GE => Ok(Binding::BooleanLiteral(a >= b)),
            Operation::LE => Ok(Binding::BooleanLiteral(a <= b)),
            Operation::EQ => Ok(Binding::BooleanLiteral(a == b)),
            Operation::NE => Ok(Binding::BooleanLiteral(a != b)),
            _ => Err(()),
        },
        _ => Err(()),
    }
}

impl Evaluate for Factor {
    fn evaluate(&self, env: &mut Environment) -> Result<Binding, ()> {
        match self {
            Factor::Expression(expr) => expr.evaluate(env),

            Factor::CallExpression(call_expr) => call_expr.evaluate(env),
            Factor::NumberLiteral(literal) => Ok(Binding::NumberLiteral(literal.value)),
            Factor::BooleanLiteral(literal) => Ok(Binding::BooleanLiteral(literal.value)),
            Factor::Var(var) => {
                // let index_eval = expr.evaluate(env)?;
                match var.expression {
                    Some(box ref expr) => {
                        let index_eval = expr.evaluate(env)?;
                        if let Binding::NumberLiteral(index) = index_eval {
                            match env.get_mut(&var.id.value).ok_or_else(|| {
                                println!(
                                    "the variable {:?} can't be found in this scope",
                                    var.id.value
                                );
                            })? {
                                Binding::Array(arr) => match arr {
                                    ArrayType::Boolean { array, .. } => {
                                        Ok(Binding::BooleanLiteral(array.borrow()[index as usize]))
                                    }
                                    ArrayType::Number { array, .. } => {
                                        Ok(Binding::NumberLiteral(array.borrow()[index as usize]))
                                    }
                                    _ => Err(()),
                                },
                                _ => {
                                    panic!("only array type can be indexed");
                                    // Err(())
                                }
                            }
                        } else {
                            panic!("index of array should be number");
                        }
                    }
                    None => env
                        .get(&var.id.value)
                        .and_then(|bind| Some(bind.clone()))
                        .ok_or_else(|| {}),
                }

                // // let index =
                //
            }
        }
    }
}
impl Evaluate for CallExpression {
    fn evaluate(&self, env: &mut Environment) -> Result<Binding, ()> {
        let func_name = &self.id.value;
        if func_name == "print" || func_name == "println" {
            let binding_list = {
                let mut vec = Vec::new();
                for arg in &self.arguments {
                    vec.push(arg.evaluate(env)?);
                }
                vec
            };
            match func_name.as_ref() {
                "print" => {
                    crate::interpreter::print(binding_list, env);
                }
                "println" => {
                    crate::interpreter::println(binding_list, env);
                }
                _ => {
                    unreachable!("only print and println");
                }
            }
            return Ok(Binding::Void);
        }
        let func_decl = env.get_func(func_name).ok_or_else(|| {})?;
        if let Binding::FunctionDeclaration(decl) = func_decl {
            let decl = decl.clone();
            env.call_expression_binding =
                prepare_call_expression_binding(env, &decl.params, &self.arguments)?;
            match decl.body.evaluate(env) {
                Ok(Some(binding)) => {
                    return Ok(binding);
                }
                Err(_) => {
                    return Err(());
                }
                _ => {
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
) -> Result<Vec<(String, Binding)>, ()> {
    match params {
        Params::ParamsList { params } => {
            if params.len() != arguments.len() {
                return Err(());
            }
            let mut array = Vec::new();
            for (param, arg) in params.iter().zip(arguments.iter()) {
                array.push((
                    param.id.value.clone(),
                    generate_assignable_binding(env, param, arg)?,
                ));
            }
            Ok(array)
        }
        Params::Void => {
            if arguments.len() != 0 {
                return Err(());
            }
            return Ok(Vec::new());
        }
    }
}

fn generate_assignable_binding(
    env: &mut Environment,
    param: &Parameter,
    arg: &Expression,
) -> Result<Binding, ()> {
    if !param.is_array {
        let arg_binding = arg.evaluate(env)?;
        match (&param.type_specifier.kind, &arg_binding) {
            (TypeSpecifierKind::Int, Binding::NumberLiteral(_)) => Ok(arg_binding),
            (TypeSpecifierKind::Boolean, Binding::BooleanLiteral(_)) => Ok(arg_binding),
            (TypeSpecifierKind::Void, Binding::Void) => Ok(arg_binding),
            _ => Err(()),
        }
    } else {
        let arg_binding = arg.evaluate(env)?;
        match (&param.type_specifier.kind, &arg_binding) {
            (TypeSpecifierKind::Int, Binding::Array(ArrayType::Number { .. })) => Ok(arg_binding),
            (TypeSpecifierKind::Boolean, Binding::Array(ArrayType::Boolean { .. })) => {
                Ok(arg_binding)
            }
            _ => Err(()),
        }
    }
}
