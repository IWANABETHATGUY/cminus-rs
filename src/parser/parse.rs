use crate::{
    error_emit::ErrorReporter,
    lexer::token::{Token, TokenType},
    parser::error::ParseError,
    parser::walk::*,
};

use std::fmt::Debug;

pub struct Parser<'a> {
    token_list: Vec<Token>,
    cursor: usize,
    source_file: &'a str,
    pub error_reporter: ErrorReporter<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(token_list: Vec<Token>, source_file: &'a str) -> Parser<'a> {
        let token_list = token_list
            .into_iter()
            .filter(|token| token.token_type != TokenType::COMMENT)
            .collect();
        let mut error_reporter = ErrorReporter::new();
        error_reporter.add_file("main.cm", source_file.to_string());
        Self {
            token_list,
            cursor: 0,
            source_file,
            error_reporter,
        }
    }

    fn get_source_file_end_range(&self) -> impl Into<std::ops::Range<usize>> {
        self.source_file.len() - 1..self.source_file.len()
    }
    fn next_token(&self) -> Option<&Token> {
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
    fn match_and_consume(&mut self, token_type: TokenType, need_report: bool) -> Result<Token, ()> {
        let token = self.next_token();
        if token.is_none() {
            let range = self.get_source_file_end_range();
            if need_report {
                self.error_reporter.add_diagnostic(
                    "main.cm",
                    range,
                    format!("expected {:?}, found none", token_type),
                );
            }
            return Err(());
        }
        let token = token.unwrap().clone();
        if token.token_type == token_type {
            self.consume(1);
            return Ok(token.clone());
        } else {
            if need_report {
                self.error_reporter.add_diagnostic(
                    "main.cm",
                    token.range(),
                    format!(
                        "expected {:?}, found {:?}",
                        token_type, token.token_type
                    ),
                );
            }
            return Err(());
        }
        // return Err(ParseError::from(format!("expected {:?}", token_type)));
    }
    fn backtrack(&mut self, step: usize) {
        self.cursor = self.cursor.wrapping_sub(step);
    }
    pub fn parse_program(&mut self) -> Result<Program, ()> {
        let mut declarations = vec![];
        while self.cursor < self.token_list.len() {
            let declaration = self.parse_declaration()?;
            declarations.push(declaration);
        }
        Ok(Program { declarations })
    }
    fn parse_declaration(&mut self) -> Result<Declaration, ()> {
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
    }

    fn parse_variable_declaration(&mut self) -> Result<Declaration, ()> {
        let type_specifier = self.parse_type_specifier()?;
        let id_token = self.match_and_consume(TokenType::ID, true)?;
        let identifier = Identifier {
            value: id_token.content,
        };
        let mut num = None;
        if self.match_token(TokenType::LBRACK) {
            self.consume(1);
            let num_token = self.match_and_consume(TokenType::NUM, true)?;
            let value = if let Ok(value) = num_token.content.parse::<i32>() {
                value
            } else {
                self.error_reporter.add_diagnostic(
                    "main.cm",
                    num_token.range(),
                    "can't parse token to integer".into(),
                );
                return Err(());
            };
            num = Some(NumberLiteral { value });
            self.match_and_consume(TokenType::RBRACK, true)?;
        }
        self.match_and_consume(TokenType::SEMI, true)?;
        Ok(Declaration::VarDeclaration(VarDeclaration {
            type_specifier,
            id: identifier,
            num,
        }))
    }

    fn parse_function_declaration(&mut self) -> Result<Declaration, ()> {
        let type_specifier = self.parse_type_specifier()?;
        let id_token = self.match_and_consume(TokenType::ID, true)?;
        let mut params: Params = Params::Void;
        let identifier = Identifier {
            value: id_token.content,
        };
        self.match_and_consume(TokenType::LPAREN, true)?;
        match self.next_token() {
            Some(token) => match token.token_type {
                TokenType::VOID => {
                    self.consume(1);
                    params = Params::Void;
                }
                // TokenType::LPAREN => {}
                _ => {
                    let mut params_list = vec![];
                    if !self.match_token(TokenType::RPAREN) {
                        params_list.push(self.parse_param()?);
                    }
                    while !self.match_token(TokenType::RPAREN) {
                        self.match_and_consume(TokenType::COMMA, true)?;
                        params_list.push(self.parse_param()?);
                    }
                    params = Params::ParamsList {
                        params: params_list,
                    }
                }
            },
            None => {}
        }
        self.match_and_consume(TokenType::RPAREN, true)?;
        let body = self.parse_compound_statement()?;
        Ok(Declaration::FunctionDeclaration(FunctionDeclaration {
            type_specifier,
            id: identifier,
            params,
            body,
        }))
    }

    fn parse_compound_statement(&mut self) -> Result<CompoundStatement, ()> {
        self.match_and_consume(TokenType::LBRACE, true)?;
        let mut local_declaration = vec![];
        let mut statement_list = vec![];
        while self.match_type_specifier() {
            match self.parse_variable_declaration() {
                Ok(decl) => match decl {
                    Declaration::FunctionDeclaration(_) => {
                        self.error_reporter.add_diagnostic(
                            "main.cm",
                            self.token_list[self.cursor].range(),
                            "Unexpected function declaration".into(),
                        );
                        return Err(());
                    }
                    Declaration::VarDeclaration(var_decl) => {
                        local_declaration.push(var_decl);
                    }
                },
                Err(_) => {
                    return Err(());
                }
            }
        }
        while !self.match_token(TokenType::RBRACE) {
            statement_list.push(self.parse_statement()?);
        }
        self.match_and_consume(TokenType::RBRACE, true)?;
        Ok(CompoundStatement {
            local_declaration,
            statement_list,
        })
    }
    fn parse_statement(&mut self) -> Result<Statement, ()> {
        match self.next_token() {
            Some(token) => match token.token_type {
                TokenType::LBRACE => Ok(Statement::CompoundStatement(
                    self.parse_compound_statement()?,
                )),
                TokenType::IF => Ok(Statement::SelectionStatement(
                    self.parse_selection_statement()?,
                )),
                TokenType::WHILE => Ok(Statement::IterationStatement(
                    self.parse_iteration_statement()?,
                )),
                TokenType::RETURN => Ok(Statement::ReturnStatement(self.parse_return_statement()?)),
                _ => Ok(self.parse_expression_statement()?),
            },
            None => {
                // return Err(ParseError::from("expected ``"));
                return Err(());
            }
        }
    }
    fn parse_iteration_statement(&mut self) -> Result<IterationStatement, ()> {
        self.match_and_consume(TokenType::WHILE, true)?;
        let expression = self.parse_expression()?;
        let body = Some(Box::new(self.parse_statement()?));
        Ok(IterationStatement {
            test: expression,
            body,
        })
    }
    fn parse_selection_statement(&mut self) -> Result<SelectionStatement, ()> {
        self.match_and_consume(TokenType::IF, true)?;
        self.match_and_consume(TokenType::LPAREN, true)?;
        let test = self.parse_expression()?;
        self.match_and_consume(TokenType::RPAREN, true)?;
        let consequent = Box::new(self.parse_statement()?);
        let alternative = if self.match_token(TokenType::ELSE) {
            self.consume(1);
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };
        Ok(SelectionStatement {
            consequent,
            alternative,
            test,
        })
    }
    fn parse_return_statement(&mut self) -> Result<ReturnStatement, ()> {
        self.match_and_consume(TokenType::RETURN, true)?;
        let mut expression = None;
        if !self.match_token(TokenType::SEMI) {
            expression = Some(self.parse_expression()?);
        }
        self.match_and_consume(TokenType::SEMI, true)?;
        Ok(ReturnStatement { expression })
    }
    fn parse_expression_statement(&mut self) -> Result<Statement, ()> {
        let mut expression = None;
        if !self.match_token(TokenType::SEMI) {
            expression = Some(self.parse_expression()?);
        }
        self.match_and_consume(TokenType::SEMI, true)?;
        Ok(Statement::ExpressionStatement(ExpressionStatement {
            expression,
        }))
    }

    fn parse_var(&mut self) -> Result<Var, ()> {
        let id = self.match_and_consume(TokenType::ID, true)?;
        let mut expression = None;
        if self.match_token(TokenType::LBRACK) {
            expression = Some(Box::new(self.parse_expression()?));
        }
        Ok(Var {
            expression,
            id: Identifier { value: id.content },
        })
    }
    fn parse_assignment_expression(&mut self) -> Result<Expression, ()> {
        let var = self.parse_var()?;
        self.match_and_consume(TokenType::ASSIGN, true)?;
        let expression = self.parse_expression()?;
        Ok(Expression::Assignment(AssignmentExpression {
            lhs: var,
            rhs: Box::new(expression),
        }))
    }
    fn parse_expression(&mut self) -> Result<Expression, ()> {
        let cursor = self.cursor;
        if let Ok(expr) = self.parse_assignment_expression() {
            return Ok(expr);
        }
        self.cursor = cursor;
        if let Ok(expr) = self.parse_simple_expression() {
            // self.error_reporter.pop_diagnostic("main.cm");
            Ok(expr)
        } else {
            // println!("parse_expression: {}", self.error_reporter.emit_string());
            Err(())
        }
    }

    fn parse_simple_expression(&mut self) -> Result<Expression, ()> {
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

    fn parse_additive_expression(&mut self) -> Result<Expression, ()> {
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

    fn parse_term(&mut self) -> Result<Expression, ()> {
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

    fn parse_factor(&mut self) -> Result<Expression, ()> {
        if let Some(token) = self.next_token() {
            let content = token.content.clone();
            let range = token.range();
            match token.token_type {
                TokenType::NUM => {
                    self.consume(1);
                    let value = if let Ok(value) = content.parse::<i32>() {
                        value
                    } else {
                        self.error_reporter.add_diagnostic(
                            "main.cm",
                            range,
                            "can't parse token to integer".into(),
                        );
                        return Err(());
                    };
                    return Ok(Expression::Factor(Factor::NumberLiteral(NumberLiteral {
                        value,
                    })));
                }
                TokenType::LPAREN => {
                    self.consume(1);
                    let expression = self.parse_expression()?;
                    self.match_and_consume(TokenType::RPAREN, true)?;
                    return Ok(Expression::Factor(Factor::Expression(Box::new(expression))));
                }
                TokenType::ID => {
                    let value = token.content.clone();
                    self.consume(1);
                    if let Some(token) = self.next_token() {
                        match token.token_type {
                            TokenType::LPAREN => {
                                self.consume(1);
                                let arguments = self.parse_args()?;
                                self.match_and_consume(TokenType::RPAREN, true)?;
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
                                self.match_and_consume(TokenType::RBRACK, true)?;
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
                _ => {
                    self.error_reporter.add_diagnostic(
                        "main.cm",
                        token.range(),
                        "expected `Identifier`, `Num`, `LPAREN`".into(),
                    );
                    return Err(());
                }
            }
        }

        self.error_reporter.add_diagnostic(
            "main.cm",
            self.get_source_file_end_range(),
            "expected Token found None".into(),
        );
        return Err(());
    }
    fn parse_args(&mut self) -> Result<Vec<Expression>, ()> {
        let mut args = vec![];
        if !self.match_token(TokenType::RPAREN) {
            args.push(self.parse_expression()?);
        }
        while !self.match_token(TokenType::RPAREN) {
            self.match_and_consume(TokenType::COMMA, true)?;
            args.push(self.parse_expression()?);
        }
        Ok(args)
    }

    fn parse_param(&mut self) -> Result<Parameter, ()> {
        let type_specifier = self.parse_type_specifier()?;
        let id_token = self.match_and_consume(TokenType::ID, true)?;
        let identifier = Identifier {
            value: id_token.content,
        };
        let mut is_array = false;
        if self.match_token(TokenType::LBRACK) {
            self.match_and_consume(TokenType::LBRACK, true)?;
            self.match_and_consume(TokenType::RBRACK, true)?;
            is_array = true;
        }
        Ok(Parameter {
            type_specifier,
            id: identifier,
            is_array,
        })
    }
    fn parse_type_specifier(&mut self) -> Result<TypeSpecifier, ()> {
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
                _ => {
                    self.error_reporter.add_diagnostic(
                        "main.cm",
                        token.range(),
                        format!("expected `int` or `void`, found {:?}", token.token_type)
                    );
                    return Err(());
                }
            }
        }
        self.error_reporter.add_diagnostic(
            "main.cm",
            self.get_source_file_end_range(),
            "expected `int` or `void`".into(),
        );
        return Err(());
    }
}
