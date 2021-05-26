use std::fmt::Display;

use codespan_drive::CodeSpan;
use serde::Serialize;
use smol_str::SmolStr;
pub trait Walk {
    fn walk(&self, level: usize) -> String;
}

pub trait Codespan {
    fn start(&self) -> usize;
    fn end(&self) -> usize;
}
#[derive(Debug, Clone, Serialize, CodeSpan)]
pub struct Program {
    pub(crate) declarations: Vec<Declaration>,
    pub start: usize,
    pub end: usize,
}
impl Walk for Program {
    fn walk(&self, level: usize) -> String {
        let ast = format!(
            "{}Program {}\n",
            " ".repeat(2 * level),
            generate_codespan_postfix(self)
        );
        let mut children = vec![];
        for decl in self.declarations.iter() {
            children.push(decl.walk(level + 1))
        }
        ast + &children.join("\n")
    }
}

#[derive(Debug, Clone, Serialize, CodeSpan)]
pub struct FunctionDeclaration {
    pub(crate) type_specifier: TypeSpecifier,
    pub(crate) id: Identifier,
    pub(crate) params: Params,
    pub(crate) body: CompoundStatement,
    pub start: usize,
    pub end: usize,
}
impl Walk for FunctionDeclaration {
    fn walk(&self, level: usize) -> String {
        let mut ast = format!(
            "{}FunctionDeclaration {}\n",
            " ".repeat(2 * level),
            generate_codespan_postfix(self)
        );
        ast += &vec![
            self.type_specifier.walk(level + 1),
            self.id.walk(level + 1),
            self.params.walk(level + 1),
            self.body.walk(level + 1),
        ]
        .join("\n");
        ast
    }
}
#[derive(Debug, Clone, Serialize, CodeSpan)]
pub struct VarDeclaration {
    pub(crate) type_specifier: TypeSpecifier,
    pub(crate) id: Identifier,
    pub(crate) num: Option<NumberLiteral>,
    pub(crate) initializer: Option<Expression>,
    pub(crate) array_initializer: Option<Vec<Expression>>,
    pub start: usize,
    pub end: usize,
}
impl Walk for VarDeclaration {
    fn walk(&self, level: usize) -> String {
        let ast = format!(
            "{}VarDeclaration {}\n",
            " ".repeat(2 * level),
            generate_codespan_postfix(self)
        );
        let mut children = vec![self.type_specifier.walk(level + 1), self.id.walk(level + 1)];
        if let Some(ref num) = self.num {
            children.push(num.walk(level + 1));
        }
        if let Some(ref initializer) = self.initializer {
            children.push(
                format!("{}<Initializer>", " ".repeat(2 * (level + 1)),)
                    + &initializer.walk(level + 1).trim_start(),
            );
        }
        if let Some(ref initializer) = self.array_initializer {
            children.push(format!("{}<Initializer>", " ".repeat(2 * (level + 1)),));
            for init in initializer {
                children.push(init.walk(level + 2));
            }
        }
        ast + &children.join("\n")
    }
}
#[derive(Debug, Clone, Serialize)]
pub enum Declaration {
    FunctionDeclaration(FunctionDeclaration),
    VarDeclaration(VarDeclaration),
}

impl Declaration {
    pub fn try_into_function_declaration(self) -> Result<FunctionDeclaration, Self> {
        if let Self::FunctionDeclaration(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}
impl Walk for Declaration {
    fn walk(&self, level: usize) -> String {
        match &self {
            Declaration::VarDeclaration(var_decl) => var_decl.walk(level),
            Declaration::FunctionDeclaration(func_decl) => func_decl.walk(level),
        }
    }
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
}
#[derive(Debug, Clone, Serialize, CodeSpan)]
pub struct Identifier {
    pub(crate) value: SmolStr,
    pub start: usize,
    pub end: usize,
}
impl Walk for Identifier {
    fn walk(&self, level: usize) -> String {
        format!(
            "{}Identifier({}) {}",
            " ".repeat(2 * level),
            self.value,
            generate_codespan_postfix(self)
        )
    }
}
#[derive(Debug, Clone, Serialize, CodeSpan)]
pub struct NumberLiteral {
    pub(crate) value: i32,
    pub start: usize,
    pub end: usize,
}
impl Walk for NumberLiteral {
    fn walk(&self, level: usize) -> String {
        format!(
            "{}NumberLiteral({}) {}",
            " ".repeat(2 * level),
            self.value,
            generate_codespan_postfix(self)
        )
    }
}
#[derive(Debug, Clone, Serialize, CodeSpan)]
pub struct BooleanLiteral {
    pub(crate) value: bool,
    pub start: usize,
    pub end: usize,
}
impl Walk for BooleanLiteral {
    fn walk(&self, level: usize) -> String {
        format!(
            "{}BooleanLiteral({}) {}",
            " ".repeat(2 * level),
            self.value,
            generate_codespan_postfix(self)
        )
    }
}
#[derive(Debug, Clone, Serialize, CodeSpan)]
pub struct TypeSpecifier {
    pub(crate) kind: TypeSpecifierKind,
    pub start: usize,
    pub end: usize,
}

impl Walk for TypeSpecifier {
    fn walk(&self, level: usize) -> String {
        format!(
            "{}TypeSpecifier({:?}) {}",
            " ".repeat(2 * level),
            self.kind,
            generate_codespan_postfix(self)
        )
    }
}
#[derive(Debug, Clone, Serialize)]
pub(crate) enum TypeSpecifierKind {
    Int,
    Void,
    Boolean,
}

#[derive(Debug, Clone, Serialize)]
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
}
impl Walk for Params {
    fn walk(&self, level: usize) -> String {
        match self {
            Params::Void => format!(
                "{}Void {}",
                " ".repeat(2 * level),
                generate_codespan_postfix(self)
            ),
            Params::ParamsList { params } => {
                let params_codespan = if params.len() > 0 {
                    let start = params[0].start;
                    let end = params[params.len() - 1].end;
                    format!("@{}..{}", start, end)
                } else {
                    "".to_string()
                };
                let ast = format!("{}ParameterList {}", " ".repeat(2 * level), params_codespan);
                if !params.is_empty() {
                    ast + "\n"
                        + &params
                            .iter()
                            .map(|param| param.walk(level + 1))
                            .filter(|param| !param.is_empty())
                            .collect::<Vec<String>>()
                            .join("\n")
                } else {
                    ast
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, CodeSpan)]
pub struct Parameter {
    pub(crate) type_specifier: TypeSpecifier,
    pub(crate) id: Identifier,
    pub(crate) is_array: bool,
    pub start: usize,
    pub end: usize,
}

impl Walk for Parameter {
    fn walk(&self, level: usize) -> String {
        format!(
            "{}Parameter({:?} {}{}) {}",
            " ".repeat(2 * level),
            self.type_specifier.kind,
            self.id.value,
            if self.is_array { "[]" } else { "" },
            generate_codespan_postfix(self)
        )
    }
}

#[derive(Debug, Clone, Serialize, CodeSpan)]
pub struct CompoundStatement {
    pub(crate) local_declaration: Vec<VarDeclaration>,
    pub(crate) statement_list: Vec<Statement>,
    pub start: usize,
    pub end: usize,
}

impl Walk for CompoundStatement {
    fn walk(&self, level: usize) -> String {
        let mut ast = format!(
            "{}CompoundStatement {}",
            " ".repeat(2 * level),
            generate_codespan_postfix(self)
        );
        if !self.local_declaration.is_empty() {
            ast = ast
                + "\n"
                + &self
                    .local_declaration
                    .iter()
                    .map(|decl| decl.walk(level + 1))
                    .filter(|item| !item.is_empty())
                    .collect::<Vec<String>>()
                    .join("\n");
        }
        if !self.statement_list.is_empty() {
            ast = ast
                + "\n"
                + &self
                    .statement_list
                    .iter()
                    .map(|stmt| stmt.walk(level + 1))
                    .filter(|item| !item.is_empty())
                    .collect::<Vec<String>>()
                    .join("\n");
        }
        ast
    }
}
#[derive(Debug, Clone, Serialize)]
pub enum Statement {
    CompoundStatement(CompoundStatement),
    ExpressionStatement(ExpressionStatement),
    SelectionStatement(SelectionStatement),
    IterationStatement(IterationStatement),
    ReturnStatement(ReturnStatement),
}

impl Walk for Statement {
    fn walk(&self, level: usize) -> String {
        match self {
            Statement::CompoundStatement(stmt) => stmt.walk(level),
            Statement::ExpressionStatement(stmt) => stmt.walk(level),
            Statement::SelectionStatement(stmt) => stmt.walk(level),
            Statement::IterationStatement(stmt) => stmt.walk(level),
            Statement::ReturnStatement(stmt) => stmt.walk(level),
        }
    }
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
}
#[derive(Debug, Clone, Serialize, CodeSpan)]
pub struct SelectionStatement {
    pub(crate) test: Expression,
    pub(crate) consequent: Box<Statement>,
    pub(crate) alternative: Option<Box<Statement>>,
    pub start: usize,
    pub end: usize,
}

impl Walk for SelectionStatement {
    fn walk(&self, level: usize) -> String {
        let ast = format!(
            "{}SelectionStatement {}\n",
            " ".repeat(2 * level),
            generate_codespan_postfix(self)
        );
        let mut children = vec![self.test.walk(level + 1), self.consequent.walk(level + 1)];
        if let Some(ref consequent) = self.alternative {
            children.push(consequent.walk(level + 1));
        }
        ast + &children
            .into_iter()
            .filter(|child| !child.is_empty())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
#[derive(Debug, Clone, Serialize, CodeSpan)]
pub struct IterationStatement {
    pub(crate) test: Expression,
    pub(crate) body: Box<Statement>,
    pub start: usize,
    pub end: usize,
}

impl Walk for IterationStatement {
    fn walk(&self, level: usize) -> String {
        let ast = format!(
            "{}IterationStatement {}\n",
            " ".repeat(2 * level),
            generate_codespan_postfix(self)
        );
        let mut children = vec![self.test.walk(level + 1)];
        let body_ast_string = self.body.walk(level + 1);
        if !body_ast_string.is_empty() {
            children.push(body_ast_string);
        }
        ast + &children.join("\n")
    }
}
#[derive(Debug, Clone, Serialize, CodeSpan)]
pub struct ReturnStatement {
    pub(crate) expression: Option<Expression>,
    pub start: usize,
    pub end: usize,
}

impl Walk for ReturnStatement {
    fn walk(&self, level: usize) -> String {
        let mut ast = format!(
            "{}ReturnStatement {}\n",
            " ".repeat(2 * level),
            generate_codespan_postfix(self)
        );
        if let Some(ref expr) = self.expression {
            ast += &expr.walk(level + 1);
        }
        ast
    }
}
#[derive(Debug, Clone, Serialize, CodeSpan)]
pub struct ExpressionStatement {
    pub(crate) expression: Option<Expression>,
    pub start: usize,
    pub end: usize,
}

impl Walk for ExpressionStatement {
    fn walk(&self, level: usize) -> String {
        if let Some(ref expr) = self.expression {
            expr.walk(level)
        } else {
            "".to_string()
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum Expression {
    Assignment(AssignmentExpression),
    BinaryExpression(BinaryExpression),
    LogicExpression(LogicExpression),
    Factor(Factor),
}

impl Walk for Expression {
    fn walk(&self, level: usize) -> String {
        match self {
            Expression::Assignment(assignment) => {
                let ast = format!(
                    "{}Assignment {}\n",
                    " ".repeat(2 * level),
                    generate_codespan_postfix(self)
                );
                ast + &assignment.walk(level)
            }
            Expression::BinaryExpression(binary_expr) => {
                let ast = format!(
                    "{}BinaryExpression {}\n",
                    " ".repeat(2 * level),
                    generate_codespan_postfix(self)
                );
                let children = vec![
                    binary_expr.left.walk(level + 1),
                    binary_expr.operation.walk(level + 1),
                    binary_expr.right.walk(level + 1),
                ];
                ast + &children.join("\n")
            }
            Expression::Factor(factor) => factor.walk(level),
            Expression::LogicExpression(logic_expr) => {
                let ast = format!(
                    "{}LogicExpression {}\n",
                    " ".repeat(2 * level),
                    generate_codespan_postfix(self)
                );
                let children = vec![
                    logic_expr.left.walk(level + 1),
                    logic_expr.operation.walk(level + 1),
                    logic_expr.right.walk(level + 1),
                ];
                ast + &children.join("\n")
            }
        }
    }
}

impl Codespan for Expression {
    fn start(&self) -> usize {
        match self {
            Expression::Assignment(expr) => expr.start,
            Expression::BinaryExpression(expr) => expr.start,
            Expression::Factor(expr) => expr.start(),
            Expression::LogicExpression(expr) => expr.start(),
        }
    }

    fn end(&self) -> usize {
        match self {
            Expression::Assignment(expr) => expr.end,
            Expression::BinaryExpression(expr) => expr.end,
            Expression::Factor(expr) => expr.end(),
            Expression::LogicExpression(expr) => expr.end(),
        }
    }
}
#[derive(Debug, Clone, Serialize, CodeSpan)]
pub struct AssignmentExpression {
    pub(crate) lhs: Var,
    pub(crate) rhs: Box<Expression>,
    pub start: usize,
    pub end: usize,
}

impl Walk for AssignmentExpression {
    fn walk(&self, level: usize) -> String {
        format!("{}\n{}", self.lhs.walk(level + 1), self.rhs.walk(level + 1))
    }
}

#[derive(Debug, Clone, Serialize, CodeSpan)]
pub struct Var {
    pub(crate) id: Identifier,
    pub(crate) expression: Option<Box<Expression>>,
    pub start: usize,
    pub end: usize,
}

impl Walk for Var {
    fn walk(&self, level: usize) -> String {
        let id = self.id.walk(level + 1);
        let mut result = vec![
            format!(
                "{}Var {}",
                " ".repeat(2 * level),
                generate_codespan_postfix(self)
            ),
            id,
        ];
        if let Some(ref expr) = self.expression {
            result.push(expr.walk(level + 1));
        }
        result.join("\n")
    }
}

#[derive(Debug, Clone, Serialize, CodeSpan)]
pub struct LogicExpression {
    pub(crate) left: Box<Expression>,
    pub(crate) right: Box<Expression>,
    pub(crate) operation: Operation,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Serialize, CodeSpan)]
pub struct BinaryExpression {
    pub(crate) left: Box<Expression>,
    pub(crate) right: Box<Expression>,
    pub(crate) operation: Operation,
    pub start: usize,
    pub end: usize,
}
#[derive(Debug, Clone, Serialize)]
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
        }
    }
}
impl Walk for Operation {
    fn walk(&self, level: usize) -> String {
        format!(
            "{}{} {}",
            " ".repeat(2 * level),
            self,
            generate_codespan_postfix(self)
        )
    }
}
#[derive(Debug, Clone, Serialize)]
pub enum Factor {
    Expression(Box<Expression>),
    Var(Var),
    CallExpression(CallExpression),
    NumberLiteral(NumberLiteral),
    BooleanLiteral(BooleanLiteral),
}

impl Walk for Factor {
    fn walk(&self, level: usize) -> String {
        match self {
            Factor::Expression(expr) => expr.walk(level),
            Factor::Var(var) => var.walk(level),
            Factor::CallExpression(call) => call.walk(level),
            Factor::NumberLiteral(num) => num.walk(level),
            Factor::BooleanLiteral(boolean) => boolean.walk(level),
        }
    }
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
}
#[derive(Debug, Clone, Serialize, CodeSpan)]
pub struct CallExpression {
    pub(crate) id: Identifier,
    pub(crate) arguments: Vec<Expression>,
    pub start: usize,
    pub end: usize,
}

impl Walk for CallExpression {
    fn walk(&self, level: usize) -> String {
        let ast = format!(
            "{}CallExpression {}\n",
            " ".repeat(level * 2),
            generate_codespan_postfix(self)
        );
        let arguments_codespan = if self.arguments.len() > 0 {
            let start = self.arguments[0].start();
            let end = self.arguments[self.arguments.len() - 1].end();
            format!("@{}..{}", start, end)
        } else {
            "".to_string()
        };
        let children = vec![
            self.id.walk(level + 1),
            format!(
                "{}Arguments {}",
                " ".repeat((level + 1) * 2),
                arguments_codespan
            ),
            self.arguments
                .iter()
                .map(|arg| arg.walk(level + 2))
                .filter(|arg| !arg.is_empty())
                .collect::<Vec<String>>()
                .join("\n"),
        ];
        ast + &children
            .into_iter()
            .filter(|item| !item.is_empty())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

fn generate_codespan_postfix<T>(node: &T) -> String
where
    T: Codespan,
{
    format!("@{}..{}", node.start(), node.end())
}
