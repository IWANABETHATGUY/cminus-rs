use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
pub enum Value {
    I32(i32),
}
impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        use Value::*;
        match (self, rhs) {
            (I32(left), I32(right)) => I32(left + right),
        }
    }
}
impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        use Value::*;
        match (self, rhs) {
            (I32(left), I32(right)) => I32(left * right),
        }
    }
}
impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        use Value::*;
        match (self, rhs) {
            (I32(left), I32(right)) => I32(left / right),
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        use Value::*;
        match (self, rhs) {
            (I32(left), I32(right)) => I32(left - right),
        }
    }
}
