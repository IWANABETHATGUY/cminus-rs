use crate::{
    lexer::token::{Token, TokenType},
    parser::error::ParseError,
};
use std::fmt::{Debug, Display};
pub trait Walk {
    fn walk(&self, level: usize);
}

pub struct Parser {
    token_list: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(token_list: Vec<Token>) -> Parser {
        Self {
            token_list,
            cursor: 0,
        }
    }
    fn next_token(&mut self) -> Option<&Token> {
        self.token_list.get(self.cursor)
    }
    fn match_token(&mut self, token_type: TokenType) -> bool {
        let next_token = self.next_token();
        if next_token.is_some() && next_token.unwrap().token_type == token_type {
            return true;
        }
        false
    }
    fn match_type_specifier(&mut self) -> bool {
        // let next_token = self.next_token();
        if let Some(token) = self.next_token() {
            return token.token_type == TokenType::VOID || token.token_type == TokenType::INT;
        }
        false
    }
    fn match_rel_op(&mut self) -> Option<Operation> {
        if let Some(token) = self.next_token() {
            match token.token_type {
                TokenType::LE => return Some(Operation::LE),
                TokenType::GE => return Some(Operation::GE),
                TokenType::GT => return Some(Operation::GT),
                TokenType::LT => return Some(Operation::LT),
                TokenType::EQ => return Some(Operation::EQ),
                TokenType::NE => return Some(Operation::NE),
                _ => return None,
            }
        }
        None
    }
    fn match_add_op(&mut self) -> Option<Operation> {
        if let Some(token) = self.next_token() {
            match token.token_type {
                TokenType::PLUS => return Some(Operation::PLUS),
                TokenType::MINUS => return Some(Operation::MINUS),
                _ => return None,
            }
        }
        None
    }
    fn match_mul_op(&mut self) -> Option<Operation> {
        if let Some(token) = self.next_token() {
            match token.token_type {
                TokenType::MULTIPLY => return Some(Operation::MULTIPLY),
                TokenType::TIMES => return Some(Operation::DIVIDE),
                _ => return None,
            }
        }
        None
    }
    fn consume(&mut self, step: usize) {
        self.cursor += step;
    }
    fn match_and_consume(&mut self, token_type: TokenType) -> Result<Token, ParseError> {
        let token = self.next_token();
        if token.is_none() {
            return Err(ParseError::from(format!("expected {:?}", token_type)));
        }
        let token = token.unwrap().clone();
        if token.token_type == token_type {
            self.consume(1);
            return Ok(token);
        }

        return Err(ParseError::from(format!("expected {:?}", token_type)));
    }
    fn backtrack(&mut self, step: usize) {
        self.cursor -= step;
    }
    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut declarations = vec![];
        while self.cursor < self.token_list.len() {
            let declaration = self.parse_declaration()?;
            declarations.push(declaration);
        }
        Ok(Program { declarations })
    }
    fn parse_declaration(&mut self) -> Result<Declaration, ParseError> {
        if self.match_token(TokenType::INT) || self.match_token(TokenType::VOID) {
            self.consume(1);
        }
        if self.match_token(TokenType::ID) {
            self.consume(1);
        }
        if self.match_token(TokenType::LPAREN) {
            self.backtrack(2);
            return self.parse_function_declaration();
        } else {
            self.backtrack(2);
            return self.parse_variable_declaration();
        }
        // return Err(ParseError::from("expected `int`,`void`"));
    }

    fn parse_variable_declaration(&mut self) -> Result<Declaration, ParseError> {
        let type_specifier = self.parse_type_specifier()?;
        let id_token = self.match_and_consume(TokenType::ID)?;
        let identifier = Identifier {
            value: id_token.content,
        };
        let mut num = None;
        if self.match_token(TokenType::LBRACK) {
            self.match_and_consume(TokenType::LBRACK)?;
            let num_token = self.match_and_consume(TokenType::NUM)?;
            num = Some(NumberLiteral {
                value: num_token.content.parse::<i32>()?,
            });
            self.match_and_consume(TokenType::RBRACK)?;
        }
        self.match_and_consume(TokenType::SEMI)?;
        Ok(Declaration::VarDeclaration(VarDeclaration {
            type_specifier,
            id: identifier,
            num,
        }))
    }
    // fn parse_variable_declaration(&self) {

    // }
    fn parse_function_declaration(&mut self) -> Result<Declaration, ParseError> {
        let type_specifier = self.parse_type_specifier()?;
        let id_token = self.match_and_consume(TokenType::ID)?;
        let mut params: Params = Params::Void;
        let identifier = Identifier {
            value: id_token.content,
        };
        self.match_and_consume(TokenType::LPAREN)?;
        match self.next_token() {
            Some(token) => match token.token_type {
                TokenType::VOID => {
                    self.consume(1);
                    params = Params::Void;
                }
                TokenType::LPAREN => {}
                _ => {
                    let mut params_list = vec![];
                    params_list.push(self.parse_param()?);
                    while !self.match_token(TokenType::RPAREN) {
                        self.match_and_consume(TokenType::COMMA)?;
                        params_list.push(self.parse_param()?);
                    }
                    params = Params::ParamsList {
                        params: params_list,
                    }
                }
            },
            None => {}
        }
        self.match_and_consume(TokenType::RPAREN)?;
        let body = self.parse_compound_statement()?;
        Ok(Declaration::FunctionDeclaration(FunctionDeclaration {
            type_specifier,
            id: identifier,
            params,
            body,
        }))
    }

    fn parse_compound_statement(&mut self) -> Result<CompoundStatement, ParseError> {
        self.match_and_consume(TokenType::LBRACE)?;
        let mut local_declaration = vec![];
        let mut statement_list = vec![];
        while self.match_type_specifier() {
            match self.parse_variable_declaration() {
                Ok(decl) => match decl {
                    Declaration::FunctionDeclaration(_) => {
                        return Err(ParseError::from("Unexpected function declaration"));
                    }
                    Declaration::VarDeclaration(var_decl) => {
                        local_declaration.push(var_decl);
                    }
                },
                Err(err) => {
                    return Err(err);
                }
            }
        }
        while !self.match_token(TokenType::RBRACE) {
            statement_list.push(self.parse_statement()?);
        }
        self.match_and_consume(TokenType::RBRACE)?;
        Ok(CompoundStatement {
            local_declaration,
            statement_list,
        })
    }
    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.next_token() {
            Some(token) => match token.token_type {
                TokenType::LBRACE => Ok(Statement::CompoundStatement(
                    self.parse_compound_statement()?,
                )),
                TokenType::IF => Ok(Statement::SelectionStatement()),
                TokenType::WHILE => Ok(Statement::IterationStatement()),
                TokenType::RETURN => Ok(Statement::ReturnStatement()),
                _ => Ok(self.parse_expression_statement()?),
            },
            None => {
                return Err(ParseError::from("expected ``"));
            }
        }
    }
    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let mut expression = None;
        if !self.match_token(TokenType::SEMI) {
            expression = Some(self.parse_expression()?);
        }
        self.match_and_consume(TokenType::SEMI)?;
        Ok(Statement::ExpressionStatement(ExpressionStatement {
            expression,
        }))
    }
    // fn parse_expression(&mut self) -> Result<Expression, ParseError> {

    //     Err(ParseError::from("expected a token, get none"))
    // }

    fn parse_var(&mut self) -> Result<Var, ParseError> {
        let id = self.match_and_consume(TokenType::ID)?;
        let mut expression = None;
        if self.match_token(TokenType::LBRACK) {
            expression = Some(Box::new(self.parse_expression()?));
        }
        Ok(Var {
            expression,
            id: Identifier { value: id.content },
        })
    }
    fn parse_assignment_expression(&mut self) -> Result<Expression, ParseError> {
        let var = self.parse_var()?;
        self.match_and_consume(TokenType::ASSIGN)?;
        let expression = self.parse_expression()?;
        Ok(Expression::Assignment(AssignmentExpression {
            lhs: var,
            rhs: Box::new(expression),
        }))
    }
    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        let cursor = self.cursor;
        match self.parse_assignment_expression() {
            Ok(expr) => {
                return Ok(expr);
            }
            Err(_) => {
                self.cursor = cursor;
            }
        }
        self.parse_simple_expression()
    }

    fn parse_simple_expression(&mut self) -> Result<Expression, ParseError> {
        let left_expr = self.parse_additive_expression()?;
        if let Some(op) = self.match_rel_op() {
            self.consume(1);
            let right_expr = self.parse_additive_expression()?;
            return Ok(Expression::BinaryExpression(BinaryExpression {
                left: Box::new(left_expr),
                right: Box::new(right_expr),
                operation: op,
            }));
        }
        Ok(left_expr)
    }

    fn parse_additive_expression(&mut self) -> Result<Expression, ParseError> {
        let left_term = self.parse_term()?;
        if let Some(operation) = self.match_add_op() {
            self.consume(1);
            let right_term = self.parse_term()?;
            return Ok(Expression::BinaryExpression(BinaryExpression {
                left: Box::new(left_term),
                right: Box::new(right_term),
                operation,
            }));
        }
        Ok(left_term)
    }

    fn parse_term(&mut self) -> Result<Expression, ParseError> {
        let left_factor = self.parse_factor()?;
        if let Some(operation) = self.match_mul_op() {
            self.consume(1);
            let right_factor = self.parse_term()?;
            return Ok(Expression::BinaryExpression(BinaryExpression {
                left: Box::new(left_factor),
                right: Box::new(right_factor),
                operation,
            }));
        }
        Ok(left_factor)
    }

    fn parse_factor(&mut self) -> Result<Expression, ParseError> {
        if let Some(token) = self.next_token() {
            let content = token.content.clone();
            match token.token_type {
                TokenType::NUM => {
                    self.consume(1);
                    return Ok(Expression::Factor(Factor::NumberLiteral(NumberLiteral {
                        value: content.parse::<i32>()?,
                    })));
                }
                TokenType::LPAREN => {
                    self.consume(1);
                    let expression = self.parse_expression()?;
                    self.match_and_consume(TokenType::RPAREN)?;
                    return Ok(expression);
                }
                TokenType::ID => {
                    let value = token.content.clone();
                    self.consume(1);
                    if let Some(token) = self.next_token() {
                        match token.token_type {
                            TokenType::LPAREN => {
                                self.consume(1);
                                let arguments = self.parse_args()?;
                                self.match_and_consume(TokenType::RPAREN)?;
                                return Ok(Expression::Factor(Factor::CallExpression(
                                    CallExpression {
                                        arguments,
                                        id: Identifier { value },
                                    },
                                )));
                            }
                            TokenType::LBRACK => {
                                self.consume(1);
                                let local_expression = self.parse_expression()?;
                                self.match_and_consume(TokenType::RBRACK)?;
                                let var = Var {
                                    id: Identifier { value },
                                    expression: Some(Box::new(local_expression)),
                                };
                                return Ok(Expression::Factor(Factor::Var(var)));
                            }
                            _ => {
                                return Ok(Expression::Factor(Factor::Var(Var {
                                    expression: None,
                                    id: Identifier { value },
                                })));
                            }
                        }
                    } else {
                        return Ok(Expression::Factor(Factor::Var(Var {
                            expression: None,
                            id: Identifier { value },
                        })));
                    }
                }
                _ => return Err(ParseError::from("expected `Identifier`, `Num`, `LPAREN`")),
            }
        }

        return Err(ParseError::from("expected Token found None"));
    }
    fn parse_args(&mut self) -> Result<Vec<Expression>, ParseError> {
        let mut args = vec![];
        if !self.match_token(TokenType::RPAREN) {
            args.push(self.parse_expression()?);
        }
        while !self.match_token(TokenType::RPAREN) {
            self.match_and_consume(TokenType::COMMA)?;
            args.push(self.parse_expression()?);
        }
        Ok(args)
    }

    fn parse_param(&mut self) -> Result<Parameter, ParseError> {
        let type_specifier = self.parse_type_specifier()?;
        let id_token = self.match_and_consume(TokenType::ID)?;
        let identifier = Identifier {
            value: id_token.content,
        };
        let mut is_array = false;
        if self.match_token(TokenType::LBRACK) {
            self.match_and_consume(TokenType::LBRACK)?;
            self.match_and_consume(TokenType::RBRACK)?;
            is_array = true;
        }
        Ok(Parameter {
            type_specifier,
            id: identifier,
            is_array,
        })
    }
    fn parse_type_specifier(&mut self) -> Result<TypeSpecifier, ParseError> {
        if let Some(token) = self.next_token() {
            match token.token_type {
                TokenType::INT => {
                    self.consume(1);
                    return Ok(TypeSpecifier {
                        kind: TypeSpecifierKind::Int,
                    });
                }
                TokenType::VOID => {
                    self.consume(1);
                    return Ok(TypeSpecifier {
                        kind: TypeSpecifierKind::Void,
                    });
                }
                _ => return Err(ParseError::from("expected `int` or `void`")),
            }
        }
        return Err(ParseError::from("expected `int` or `void`"));
    }
}

#[derive(Debug)]
pub struct Program {
    declarations: Vec<Declaration>,
}
impl Walk for Program {
    fn walk(&self, level: usize) {
        println!("{}Program", " ".repeat(2 * level));
        for decl in self.declarations.iter() {
            decl.walk(level + 1);
        }
    }
}
// impl Debug for Program {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

//     }

// }
#[derive(Debug)]
pub struct FunctionDeclaration {
    type_specifier: TypeSpecifier,
    id: Identifier,
    params: Params,
    body: CompoundStatement,
}
impl Walk for FunctionDeclaration {
    fn walk(&self, level: usize) {
        println!("{}FunctionDeclaration", " ".repeat(2 * level));
        self.type_specifier.walk(level + 1);
        self.id.walk(level + 1);
        self.params.walk(level + 1);
        self.body.walk(level + 1);
    }
}
#[derive(Debug)]
pub struct VarDeclaration {
    type_specifier: TypeSpecifier,
    id: Identifier,
    num: Option<NumberLiteral>,
}
impl Walk for VarDeclaration {
    fn walk(&self, level: usize) {
        println!("{}VarDeclaration", " ".repeat(2 * level));
        self.id.walk(level + 1);
        self.type_specifier.walk(level + 1);
        if let Some(ref num) = self.num {
            num.walk(level + 1);
        }
    }
}
#[derive(Debug)]
pub enum Declaration {
    FunctionDeclaration(FunctionDeclaration),
    VarDeclaration(VarDeclaration),
}
impl Walk for Declaration {
    fn walk(&self, level: usize) {
        match &self {
            Declaration::VarDeclaration(var_decl) => {
                var_decl.walk(level);
            }
            Declaration::FunctionDeclaration(func_decl) => {
                func_decl.walk(level);
            }
        }
    }
}
#[derive(Debug)]
pub struct Identifier {
    value: String,
}
impl Walk for Identifier {
    fn walk(&self, level: usize) {
        println!("{}Identifier({})", " ".repeat(2 * level), self.value);
    }
}
#[derive(Debug)]
pub struct NumberLiteral {
    value: i32,
}
impl Walk for NumberLiteral {
    fn walk(&self, level: usize) {
        println!("{}NumberLiteral({})", " ".repeat(2 * level), self.value);
    }
}
#[derive(Debug)]
pub struct TypeSpecifier {
    kind: TypeSpecifierKind,
}

impl Walk for TypeSpecifier {
    fn walk(&self, level: usize) {
        println!("{}TypeSpecifier({:?})", " ".repeat(2 * level), self.kind);
    }
}
#[derive(Debug)]
enum TypeSpecifierKind {
    Int,
    Void,
}
#[derive(Debug)]
pub enum Params {
    Void,
    ParamsList { params: Vec<Parameter> },
}

impl Walk for Params {
    fn walk(&self, level: usize) {
        match self {
            Params::Void => {
                println!("{}Void", " ".repeat(2 * level));
            }
            Params::ParamsList { params } => {
                println!("{}ParameterList", " ".repeat(2 * level));
                for param in params.iter() {
                    param.walk(level + 1);
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Parameter {
    type_specifier: TypeSpecifier,
    id: Identifier,
    is_array: bool,
}

impl Walk for Parameter {
    fn walk(&self, level: usize) {
        println!(
            "{}Parameter({:?} {}{})",
            " ".repeat(2 * level),
            self.type_specifier.kind,
            self.id.value,
            if self.is_array { "[]" } else { "" }
        );
    }
}

#[derive(Debug)]
pub struct CompoundStatement {
    local_declaration: Vec<VarDeclaration>,
    statement_list: Vec<Statement>,
}

impl Walk for CompoundStatement {
    fn walk(&self, level: usize) {
        println!("{}CompoundStatement", " ".repeat(2 * level));
        for var_decl in self.local_declaration.iter() {
            var_decl.walk(level + 1);
        }
        for statement in self.statement_list.iter() {
            statement.walk(level + 1);
        }
    }
}
#[derive(Debug)]
pub enum Statement {
    CompoundStatement(CompoundStatement),
    ExpressionStatement(ExpressionStatement),
    SelectionStatement(),
    IterationStatement(),
    ReturnStatement(),
}

impl Walk for Statement {
    fn walk(&self, level: usize) {
        match self {
            Statement::CompoundStatement(stmt) => {
                stmt.walk(level);
            }
            Statement::ExpressionStatement(stmt) => {
                stmt.walk(level);
            }
            Statement::SelectionStatement() => {}
            Statement::IterationStatement() => {}
            Statement::ReturnStatement() => {}
        }
    }
}
#[derive(Debug)]
pub struct ExpressionStatement {
    expression: Option<Expression>,
}

impl Walk for ExpressionStatement {
    fn walk(&self, level: usize) {
        if let Some(ref expr) = self.expression {
            expr.walk(level);
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Assignment(AssignmentExpression),
    BinaryExpression(BinaryExpression),
    Factor(Factor),
}

impl Walk for Expression {
    fn walk(&self, level: usize) {
        match self {
            Expression::Assignment(assignment) => {
                println!("{}Assignment", " ".repeat(2 * level));
                assignment.walk(level);
            }
            Expression::BinaryExpression(binary_expr) => {
                println!("{}BinaryExpression", " ".repeat(2 * level));
                binary_expr.left.walk(level + 1);
                binary_expr.operation.walk(level + 1);
                binary_expr.right.walk(level + 1);
            }
            Expression::Factor(factor) => {
                factor.walk(level);
            }
        }
    }
}
#[derive(Debug)]
pub struct AssignmentExpression {
    lhs: Var,
    rhs: Box<Expression>,
}

impl Walk for AssignmentExpression {
    fn walk(&self, level: usize) {
        self.lhs.walk(level + 1);
        self.rhs.walk(level + 1);
    }
}

#[derive(Debug)]
pub struct Var {
    id: Identifier,
    expression: Option<Box<Expression>>,
}

impl Walk for Var {
    fn walk(&self, level: usize) {
        self.id.walk(level);
    }
}

#[derive(Debug)]
pub struct BinaryExpression {
    left: Box<Expression>,
    right: Box<Expression>,
    operation: Operation,
}
#[derive(Debug)]
pub enum Operation {
    GT,
    LT,
    GE,
    LE,
    EQ,
    NE,
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
}

impl Walk for Operation {
    fn walk(&self, level: usize) {
        println!("{}{:?}", " ".repeat(2 * level), self);
    }
}
#[derive(Debug)]
pub enum Factor {
    Expression(Box<Expression>),
    Var(Var),
    CallExpression(CallExpression),
    NumberLiteral(NumberLiteral),
}

impl Walk for Factor {
    fn walk(&self, level: usize) {
        match self {
            Factor::Expression(expr) => {
                expr.walk(level);
            }
            Factor::Var(var) => {
                var.walk(level);
            }
            Factor::CallExpression(call) => {
                call.walk(level);
            }
            Factor::NumberLiteral(num) => {
                num.walk(level);
            }
        }
    }
}
#[derive(Debug)]
pub struct CallExpression {
    id: Identifier,
    arguments: Vec<Expression>,
}

impl Walk for CallExpression {
    fn walk(&self, level: usize) {
        println!("{}CallExpression", " ".repeat(level * 2));
        self.id.walk(level + 1);
        println!("{}Arguments", " ".repeat((level + 1) * 2));
        for arg in self.arguments.iter() {
            arg.walk(level + 2);
        }
    }
}
