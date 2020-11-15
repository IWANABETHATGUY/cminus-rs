use crate::parser::ast::FunctionDeclaration;
use std::collections::HashMap;
#[derive(Debug)]
pub enum LiteralType {
    Boolean(bool),
    Number(i32),
}

#[derive(Debug)]
pub enum ArrayType {
    Boolean { length: usize, array: Vec<bool> },
    Number { length: usize, array: Vec<i32> },
}

impl ArrayType {
    pub fn get(&self, i: usize) -> Result<LiteralType, String> {
        match self {
            ArrayType::Boolean {
                length,
                array: value,
            } => {
                if i >= *length || i < 0 {
                    Err("the index should less than array length and greater than 0".into())
                } else {
                    Ok(LiteralType::Boolean(value[i]))
                }
            }
            ArrayType::Number {
                length,
                array: value,
            } => {
                if i >= *length || i < 0 {
                    Err("the index should less than array length and greater than 0".into())
                } else {
                    Ok(LiteralType::Number(value[i]))
                }
            }
        }
    }

    pub fn set(&mut self, i: usize, data: LiteralType) -> Result<(), String> {
        match self {
            ArrayType::Boolean { length, array } if matches!(data, LiteralType::Boolean(_)) => {
                if i >= *length || i < 0 {
                    Err("the index should less than array length and greater than 0".into())
                } else {
                    if let LiteralType::Boolean(v) = data {
                        array[i] = v;
                    }
                    Ok(())
                }
            }
            ArrayType::Number { length, array } if matches!(data, LiteralType::Number(_)) => {
                if i >= *length || i < 0 {
                    Err("the index should less than array length and greater than 0".into())
                } else {
                    if let LiteralType::Number(v) = data {
                        array[i] = v;
                    }
                    Ok(())
                }
            }
            _ => Err("type doesn't matched".into()),
        }
    }
}
#[derive(Debug)]
pub enum Binding {
    Literal(LiteralType),
    Array(ArrayType),
    FunctionDeclaration(Box<FunctionDeclaration>),
    Variable(String),
    Void
}
type Scope = HashMap<String, Binding>;
#[derive(Debug)]
pub struct Environment {
    pub(crate) scope_stack: Vec<Scope>,
}
