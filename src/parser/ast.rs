use std::fmt::Display;

use codespan_drive::CodeSpan;
use smol_str::SmolStr;
use super::span::Codespan;

#[derive(Debug, Clone, CodeSpan)]
pub struct Program {
    pub(crate) declarations: Vec<Declaration>,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, CodeSpan)]
pub struct FunctionDeclaration {
    pub(crate) type_specifier: TypeSpecifier,
    pub(crate) id: Identifier,
    pub(crate) params: Params,
    pub(crate) body: CompoundStatement,
    pub start: usize,
    pub end: usize,
}
#[derive(Debug, Clone, CodeSpan)]
pub struct VarDeclaration {
    pub(crate) type_specifier: TypeSpecifier,
    pub(crate) id: Identifier,
    pub(crate) num: Option<NumberLiteral>,
    pub(crate) initializer: Option<Expression>,
    pub(crate) array_initializer: Option<Vec<Expression>>,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub enum Declaration {
    FunctionDeclaration(FunctionDeclaration),
    VarDeclaration(VarDeclaration),
}

impl Codespan for Declaration {
    fn start(&self) -> usize {
        match self {
            Declaration::FunctionDeclaration(decl) => decl.start,
            Declaration::VarDeclaration(decl) => decl.start,
        }
    }

    fn end(&self) -> usize {
        match self {
            Declaration::FunctionDeclaration(decl) => decl.end,
            Declaration::VarDeclaration(decl) => decl.end,
        }
    }

    fn set_start(&mut self, start: usize) {
        match self {
            Declaration::FunctionDeclaration(decl) => decl.start = start,
            Declaration::VarDeclaration(decl) => decl.start = start,
        };
    }

    fn set_end(&mut self, end: usize) {
        match self {
            Declaration::FunctionDeclaration(decl) => decl.end = end,
            Declaration::VarDeclaration(decl) => decl.end = end,
        };
    }
}
#[derive(Debug, Clone, CodeSpan)]
pub struct Identifier {
    pub(crate) value: SmolStr,
    pub start: usize,
    pub end: usize,
}
#[derive(Debug, Clone, CodeSpan)]
pub struct NumberLiteral {
    pub(crate) value: i32,
    pub start: usize,
    pub end: usize,
}
#[derive(Debug, Clone, CodeSpan)]
pub struct BooleanLiteral {
    pub(crate) value: bool,
    pub start: usize,
    pub end: usize,
}
#[derive(Debug, Clone, CodeSpan)]
pub struct TypeSpecifier {
    pub(crate) kind: TypeSpecifierKind,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub(crate) enum TypeSpecifierKind {
    Int,
    Void,
    Boolean,
}

#[derive(Debug, Clone)]
pub enum Params {
    Void,
    ParamsList { params: Vec<Parameter> },
}

impl Codespan for Params {
    fn start(&self) -> usize {
        match self {
            Params::Void => self.start(),
            Params::ParamsList { .. } => {
                unreachable!()
            }
        }
    }

    fn end(&self) -> usize {
        match self {
            Params::Void => self.end(),
            Params::ParamsList { .. } => {
                unreachable!()
            }
        }
    }

    fn set_start(&mut self, _start: usize) {
        unimplemented!() // TODO
    }

    fn set_end(&mut self, _end: usize) {
        unimplemented!() // TODO
    }
}

#[derive(Debug, Clone, CodeSpan)]
pub struct Parameter {
    pub(crate) type_specifier: TypeSpecifier,
    pub(crate) id: Identifier,
    pub(crate) is_array: bool,
    pub start: usize,
    pub end: usize,
}


#[derive(Debug, Clone, CodeSpan)]
pub struct CompoundStatement {
    pub(crate) local_declaration: Vec<VarDeclaration>,
    pub(crate) statement_list: Vec<Statement>,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub enum Statement {
    CompoundStatement(CompoundStatement),
    ExpressionStatement(ExpressionStatement),
    SelectionStatement(SelectionStatement),
    IterationStatement(IterationStatement),
    ReturnStatement(ReturnStatement),
}

impl Codespan for Statement {
    fn start(&self) -> usize {
        match self {
            Statement::CompoundStatement(stmt) => stmt.start,
            Statement::ExpressionStatement(stmt) => stmt.start,
            Statement::SelectionStatement(stmt) => stmt.start,
            Statement::IterationStatement(stmt) => stmt.start,
            Statement::ReturnStatement(stmt) => stmt.start,
        }
    }

    fn end(&self) -> usize {
        match self {
            Statement::CompoundStatement(stmt) => stmt.end,
            Statement::ExpressionStatement(stmt) => stmt.end,
            Statement::SelectionStatement(stmt) => stmt.end,
            Statement::IterationStatement(stmt) => stmt.end,
            Statement::ReturnStatement(stmt) => stmt.end,
        }
    }

    fn set_start(&mut self, _start: usize) {}

    fn set_end(&mut self, _end: usize) {}
}
#[derive(Debug, Clone, CodeSpan)]
pub struct SelectionStatement {
    pub(crate) test: Expression,
    pub(crate) consequent: Box<Statement>,
    pub(crate) alternative: Option<Box<Statement>>,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, CodeSpan)]
pub struct IterationStatement {
    pub(crate) test: Expression,
    pub(crate) body: Box<Statement>,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, CodeSpan)]
pub struct ReturnStatement {
    pub(crate) expression: Option<Expression>,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, CodeSpan)]
pub struct ExpressionStatement {
    pub(crate) expression: Option<Expression>,
    pub start: usize,
    pub end: usize,
}


#[derive(Debug, Clone)]
pub enum Expression {
    Assignment(AssignmentExpression),
    BinaryExpression(BinaryExpression),
    LogicExpression(LogicExpression),
    UnaryExpression(UnaryExpression),
    Factor(Factor),
}


impl Codespan for Expression {
    fn start(&self) -> usize {
        match self {
            Expression::Assignment(expr) => expr.start,
            Expression::BinaryExpression(expr) => expr.start,
            Expression::Factor(expr) => expr.start(),
            Expression::LogicExpression(expr) => expr.start(),
            Expression::UnaryExpression(expr) => expr.start(),
        }
    }

    fn end(&self) -> usize {
        match self {
            Expression::Assignment(expr) => expr.end,
            Expression::BinaryExpression(expr) => expr.end,
            Expression::Factor(expr) => expr.end(),
            Expression::LogicExpression(expr) => expr.end(),
            Expression::UnaryExpression(expr) => expr.end(),
        }
    }

    fn set_start(&mut self, start: usize) {
        match self {
            Expression::Assignment(expr) => expr.start = start,
            Expression::BinaryExpression(expr) => expr.start = start,
            Expression::Factor(_) => unimplemented!(),
            Expression::LogicExpression(expr) => expr.start = start,
            Expression::UnaryExpression(_) => unimplemented!(),
        };
    }

    fn set_end(&mut self, end: usize) {
        match self {
            Expression::Assignment(expr) => expr.end = end,
            Expression::BinaryExpression(expr) => expr.end = end,
            Expression::Factor(_) => unimplemented!(),
            Expression::LogicExpression(expr) => expr.end = end,
            Expression::UnaryExpression(_) => unimplemented!(),
        };
    }
}
#[derive(Debug, Clone, CodeSpan)]
pub struct AssignmentExpression {
    pub(crate) lhs: Var,
    pub(crate) rhs: Box<Expression>,
    pub start: usize,
    pub end: usize,
}


#[derive(Debug, Clone, CodeSpan)]
pub struct Var {
    pub(crate) id: Identifier,
    pub(crate) expression: Option<Box<Expression>>,
    pub start: usize,
    pub end: usize,
}


#[derive(Debug, Clone, CodeSpan)]
pub struct LogicExpression {
    pub(crate) left: Box<Expression>,
    pub(crate) right: Box<Expression>,
    pub(crate) operation: Operation,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, CodeSpan)]
pub struct UnaryExpression {
    pub(crate) expression: Box<Expression>,
    pub(crate) operation: Operation,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, CodeSpan)]
pub struct BinaryExpression {
    pub(crate) left: Box<Expression>,
    pub(crate) right: Box<Expression>,
    pub(crate) operation: Operation,
    pub start: usize,
    pub end: usize,
}
#[derive(Debug, Clone)]
pub enum Operation {
    GT(usize, usize),
    LT(usize, usize),
    GE(usize, usize),
    LE(usize, usize),
    EQ(usize, usize),
    NE(usize, usize),
    PLUS(usize, usize),
    MINUS(usize, usize),
    MULTIPLY(usize, usize),
    DIVIDE(usize, usize),
    AND(usize, usize),
    OR(usize, usize),
    NEG(usize, usize),
    POS(usize, usize),
}
impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::GT(_, _) => {
                write!(f, "GT")
            }
            Operation::LT(_, _) => {
                write!(f, "LT")
            }
            Operation::GE(_, _) => {
                write!(f, "GE")
            }
            Operation::LE(_, _) => {
                write!(f, "LE")
            }
            Operation::EQ(_, _) => {
                write!(f, "EQ")
            }
            Operation::NE(_, _) => {
                write!(f, "NE")
            }
            Operation::PLUS(_, _) => {
                write!(f, "PLUS")
            }
            Operation::MINUS(_, _) => {
                write!(f, "MINUS")
            }
            Operation::MULTIPLY(_, _) => {
                write!(f, "MULTIPLY")
            }
            Operation::DIVIDE(_, _) => {
                write!(f, "DIVIDE")
            }
            Operation::AND(_, _) => {
                write!(f, "AND")
            }
            Operation::OR(_, _) => {
                write!(f, "OR")
            }
            Operation::NEG(_, _) => {
                write!(f, "NEG")
            }
            Operation::POS(_, _) => write!(f, "POS"),
        }
    }
}
impl Codespan for Operation {
    fn start(&self) -> usize {
        match self {
            Operation::GT(start, _) => *start,
            Operation::LT(start, _) => *start,
            Operation::GE(start, _) => *start,
            Operation::LE(start, _) => *start,
            Operation::EQ(start, _) => *start,
            Operation::NE(start, _) => *start,
            Operation::PLUS(start, _) => *start,
            Operation::MINUS(start, _) => *start,
            Operation::MULTIPLY(start, _) => *start,
            Operation::DIVIDE(start, _) => *start,
            Operation::AND(start, _) => *start,
            Operation::OR(start, _) => *start,
            Operation::NEG(start, _) => *start,
            Operation::POS(start, _) => *start,
        }
    }

    fn end(&self) -> usize {
        match self {
            Operation::GT(_, end) => *end,
            Operation::LT(_, end) => *end,
            Operation::GE(_, end) => *end,
            Operation::LE(_, end) => *end,
            Operation::EQ(_, end) => *end,
            Operation::NE(_, end) => *end,
            Operation::PLUS(_, end) => *end,
            Operation::MINUS(_, end) => *end,
            Operation::MULTIPLY(_, end) => *end,
            Operation::DIVIDE(_, end) => *end,
            Operation::AND(_, end) => *end,
            Operation::OR(_, end) => *end,
            Operation::NEG(_, end) => *end,
            Operation::POS(_, end) => *end,
        }
    }

    fn set_end(&mut self, _end: usize) {
        unimplemented!() // TODO
    }
    fn set_start(&mut self, _start: usize) {
        unimplemented!() // TODO
    }
}
#[derive(Debug, Clone)]
pub enum Factor {
    Expression(Box<Expression>),
    Var(Var),
    CallExpression(CallExpression),
    NumberLiteral(NumberLiteral),
    BooleanLiteral(BooleanLiteral),
}


impl Codespan for Factor {
    fn start(&self) -> usize {
        match self {
            Factor::Expression(expr) => expr.start(),
            Factor::Var(var) => var.start,
            Factor::CallExpression(call_expression) => call_expression.start,
            Factor::NumberLiteral(num) => num.start,
            Factor::BooleanLiteral(boolean) => boolean.start,
        }
    }

    fn end(&self) -> usize {
        match self {
            Factor::Expression(expr) => expr.end(),
            Factor::Var(var) => var.end,
            Factor::CallExpression(call_expression) => call_expression.end,
            Factor::NumberLiteral(num) => num.end,
            Factor::BooleanLiteral(boolean) => boolean.end,
        }
    }

    fn set_start(&mut self, start: usize) {
        match self {
            Factor::Expression(expr) => unimplemented!(),
            Factor::Var(var) => var.start = start,
            Factor::CallExpression(call_expression) => call_expression.start = start,
            Factor::NumberLiteral(num) => num.start = start,
            Factor::BooleanLiteral(boolean) => boolean.start = start,
        };
    }

    fn set_end(&mut self, end: usize) {
        match self {
            Factor::Expression(expr) => unimplemented!(),
            Factor::Var(var) => var.end = end,
            Factor::CallExpression(call_expression) => call_expression.end = end,
            Factor::NumberLiteral(num) => num.end = end,
            Factor::BooleanLiteral(boolean) => boolean.end = end,
        };
    }
}
#[derive(Debug, Clone, CodeSpan)]
pub struct CallExpression {
    pub(crate) id: Identifier,
    pub(crate) arguments: Vec<Expression>,
    pub start: usize,
    pub end: usize,
}
