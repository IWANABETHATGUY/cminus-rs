use crate::parser::ast::FunctionDeclaration;
use enum_as_inner::EnumAsInner;
use fxhash::FxHashMap;

use std::{cell::RefCell, rc::Rc};
#[derive(Debug, Clone)]
pub enum LiteralType {
    Boolean(bool),
    Number(i32),
}

pub trait IntoLiteral {
    fn get_literal(&self) -> LiteralType;
}

impl IntoLiteral for i32 {
    fn get_literal(&self) -> LiteralType {
        LiteralType::Number(*self)
    }
}
impl IntoLiteral for bool {
    fn get_literal(&self) -> LiteralType {
        LiteralType::Boolean(*self)
    }
}
impl IntoLiteral for Binding {
    fn get_literal(&self) -> LiteralType {
        match self {
            Binding::NumberLiteral(n) => LiteralType::Number(*n),
            Binding::BooleanLiteral(b) => LiteralType::Boolean(*b),
            _ => {
                unimplemented!() // TODO
            }
        }
    }
}
#[derive(Debug, Clone, EnumAsInner)]
pub enum ArrayType {
    Boolean {
        length: usize,
        array: Rc<RefCell<Vec<bool>>>,
    },
    Number {
        length: usize,
        array: Rc<RefCell<Vec<i32>>>,
    },
}

impl ArrayType {
    pub fn get(&self, i: usize) -> Result<LiteralType, String> {
        match self {
            ArrayType::Boolean {
                length,
                array: value,
            } => {
                if i >= *length {
                    Err("the index should less than array length and greater than 0".into())
                } else {
                    Ok(LiteralType::Boolean(value.borrow()[i]))
                }
            }
            ArrayType::Number {
                length,
                array: value,
            } => {
                if i >= *length {
                    Err("the index should less than array length and greater than 0".into())
                } else {
                    Ok(LiteralType::Number(value.borrow()[i]))
                }
            }
        }
    }

    pub fn set(&mut self, i: usize, data: LiteralType) -> Result<(), String> {
        match self {
            ArrayType::Boolean { length, array } if matches!(data, LiteralType::Boolean(_)) => {
                if i >= *length {
                    Err("the index should less than array length and greater than 0".into())
                } else {
                    if let LiteralType::Boolean(v) = data {
                        array.borrow_mut()[i] = v;
                    }
                    Ok(())
                }
            }
            ArrayType::Number { length, array } if matches!(data, LiteralType::Number(_)) => {
                if i >= *length {
                    Err("the index should less than array length and greater than 0".into())
                } else {
                    if let LiteralType::Number(v) = data {
                        array.borrow_mut()[i] = v;
                    }
                    Ok(())
                }
            }
            _ => Err("type doesn't matched".into()),
        }
    }
}
#[derive(Debug, Clone, EnumAsInner)]
pub enum Binding {
    NumberLiteral(i32),
    BooleanLiteral(bool),
    Array(ArrayType),
    FunctionDeclaration(Rc<FunctionDeclaration>),
    Variable(String),
    Void,
}
pub type Scope = FxHashMap<String, Binding>;
#[derive(Debug)]
pub struct Environment {
    pub(crate) scope_stack: Vec<Scope>,
    pub(crate) call_expression_binding: Vec<(String, Binding)>,
    pub(crate) std_io: bool,
    pub(crate) std_simulator: Vec<String>,
}

impl Environment {
    pub fn get(&self, name: &String) -> Option<&Binding> {
        for scope in self.scope_stack.iter().rev() {
            if let Some(binding) = scope.get(name) {
                return Some(binding);
            }
        }
        None
    }

    pub fn get_func(&self, name: &String) -> Option<&Binding> {
        if let Some(binding) = self.scope_stack.first().unwrap().get(name) {
            return Some(binding);
        }
        None
    }
    pub fn get_mut(&mut self, name: &String) -> Option<&mut Binding> {
        for scope in self.scope_stack.iter_mut().rev() {
            if let Some(binding) = scope.get_mut(name) {
                return Some(binding);
            }
        }
        None
    }
    pub fn define(&mut self, name: String, binding: Binding) -> Result<(), ()> {
        if let Some(scope) = self.scope_stack.last_mut() {
            if scope.contains_key(&name) {
                return Err(());
            } else {
                scope.insert(name, binding);
                return Ok(());
            }
        }
        Err(())
    }
    pub fn get_std_simulator_string(&self) -> String {
        format!("{}", self.std_simulator.join(""))
    }
    // pub fn update(&mut self, name: &String) {

    // }
}
