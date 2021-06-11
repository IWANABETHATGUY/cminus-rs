use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Eq, PartialEq, Clone, PartialOrd, Copy)]
pub enum Value {
    I32(i32),
    Boolean(bool),
    Nil
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::I32(v) => write!(f, "integer({})", v),
            Value::Boolean(v) => write!(f, "boolean({})", v),
            Value::Nil => write!(f, "nil"),
        }
    }
}
impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        use Value::*;
        match (self, rhs) {
            (I32(left), I32(right)) => I32(left + right),
            (a, b) => unreachable!("{} can't be added to {}", a, b)
        }
    }
}
impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        use Value::*;
        match (self, rhs) {
            (I32(left), I32(right)) => I32(left * right),
            (a, b) => unreachable!("{} can't be multiplied to {}", a, b)
        }
    }
}
impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        use Value::*;
        match (self, rhs) {
            (I32(left), I32(right)) => I32(left / right),
            (a, b) => unreachable!("{} can't be divided to {}", a, b)
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        use Value::*;
        match (self, rhs) {
            (I32(left), I32(right)) => I32(left - right),
            (a, b) => unreachable!("{} can't be subtracted to {}", a, b)
        }
    }
}

