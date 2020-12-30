use crate::{
    error_emit::ErrorReporter,
    lexer::token::{KeywordType, Token, TokenType},
    parser::ast::*,
};

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
            .filter(|token| token.token_type != TokenType::Comment)
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
            token.token_type == TokenType::Keyword(KeywordType::VOID)
                || token.token_type == TokenType::Keyword(KeywordType::INT)
                || token.token_type == TokenType::Keyword(KeywordType::BOOL)
        } else {
            false
        }
    }
    fn match_rel_op(&mut self) -> Option<Operation> {
        if let Some(token) = self.next_token() {
            let start = token.start_index;
            let end = token.end_index;
            match token.token_type {
                TokenType::Le => return Some(Operation::LE(start, end)),
                TokenType::Ge => return Some(Operation::GE(start, end)),
                TokenType::Gt => return Some(Operation::GT(start, end)),
                TokenType::Lt => return Some(Operation::LT(start, end)),
                TokenType::Eq => return Some(Operation::EQ(start, end)),
                TokenType::Ne => return Some(Operation::NE(start, end)),
                _ => return None,
            }
        }
        None
    }
    fn match_add_op(&mut self) -> Option<Operation> {
        if let Some(token) = self.next_token() {
            let start = token.start_index;
            let end = token.end_index;
            match token.token_type {
                TokenType::Plus => return Some(Operation::PLUS(start, end)),
                TokenType::Minus => return Some(Operation::MINUS(start, end)),
                _ => return None,
            }
        }
        None
    }
    fn match_mul_op(&mut self) -> Option<Operation> {
        if let Some(token) = self.next_token() {
            let start = token.start_index;
            let end = token.end_index;
            match token.token_type {
                TokenType::Multiply => return Some(Operation::MULTIPLY(start, end)),
                TokenType::Times => return Some(Operation::DIVIDE(start, end)),
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
                    format!("expected {}, found none", token_type),
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
                    format!("expected {}, found {}", token_type, token.token_type),
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
        let mut end = 0;
        while self.cursor < self.token_list.len() {
            let declaration = self.parse_declaration()?;
            end = declaration.end();
            declarations.push(declaration);
        }
        Ok(Program {
            declarations,
            start: 0,
            end,
        })
    }

    fn parse_declaration(&mut self) -> Result<Declaration, ()> {
        if self.match_type_specifier() {
            self.consume(1);
        } else {
            let range = self.next_token().ok_or_else(|| ())?.range();
            self.error_reporter.add_diagnostic(
                "main.cm",
                range,
                format!(
                    "expected `int` , `bool` or `void` , found {}",
                    self.next_token().unwrap().token_type
                ),
            );
            return Err(());
        }
        self.match_and_consume(TokenType::Id, true)?;
        if self.match_token(TokenType::Lparen) {
            self.backtrack(2);
            return self.parse_function_declaration();
        } else {
            self.backtrack(2);
            return self.parse_variable_declaration();
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<Declaration, ()> {
        let type_specifier = self.parse_type_specifier()?;
        let id_token = self.match_and_consume(TokenType::Id, true)?;
        let identifier = Identifier {
            value: id_token.content,
            start: id_token.start_index,
            end: id_token.end_index,
        };
        let mut num = None;
        let mut array_initializer = None;
        if self.match_token(TokenType::Lbrack) {
            self.consume(1);
            let num_token = self.match_and_consume(TokenType::NumberLiteral, true)?;
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
            num = Some(NumberLiteral {
                value,
                start: num_token.start_index,
                end: num_token.end_index,
            });
            self.match_and_consume(TokenType::Rbrack, true)?;
        }
        let mut initializer = None;
        if self.match_token(TokenType::Assign) {
            self.consume(1);
            if num.is_some() {
                array_initializer = Some(self.parse_array_initialization()?);
            } else {
                initializer = Some(self.parse_expression()?);
            }
        }
        let end = (self.match_and_consume(TokenType::Semi, true)?).end_index;
        Ok(Declaration::VarDeclaration(VarDeclaration {
            start: type_specifier.start,
            end,
            type_specifier,
            id: identifier,
            initializer,
            array_initializer,
            num,
        }))
    }
    fn parse_array_initialization(&mut self) -> Result<Vec<Expression>, ()> {
        let mut expressions = vec![];
        self.match_and_consume(TokenType::Lbrace, true)?;
        expressions.push(self.parse_expression()?);
        while self.match_token(TokenType::Comma) {
            self.consume(1);
            expressions.push(self.parse_expression()?);
        }
        self.match_and_consume(TokenType::Rbrace, true)?;
        Ok(expressions)
    }
    fn parse_function_declaration(&mut self) -> Result<Declaration, ()> {
        let type_specifier = self.parse_type_specifier()?;
        let id_token = self.match_and_consume(TokenType::Id, true)?;
        let mut params: Params = Params::Void;
        let identifier = Identifier {
            value: id_token.content,
            start: id_token.start_index,
            end: id_token.end_index,
        };
        self.match_and_consume(TokenType::Lparen, true)?;
        match self.next_token() {
            Some(token) => match token.token_type {
                TokenType::Keyword(KeywordType::VOID) => {
                    self.consume(1);
                    params = Params::Void;
                }
                // TokenType::LPAREN => {}
                _ => {
                    let mut params_list = vec![];
                    if !self.match_token(TokenType::Rparen) {
                        params_list.push(self.parse_param()?);
                    }
                    while !self.match_token(TokenType::Rparen) {
                        self.match_and_consume(TokenType::Comma, true)?;
                        params_list.push(self.parse_param()?);
                    }
                    params = Params::ParamsList {
                        params: params_list,
                    }
                }
            },
            None => {}
        }
        self.match_and_consume(TokenType::Rparen, true)?;
        let body = self.parse_compound_statement()?;
        Ok(Declaration::FunctionDeclaration(FunctionDeclaration {
            start: type_specifier.start,
            end: body.end,
            id: identifier,
            type_specifier,
            params,
            body,
        }))
    }

    fn parse_compound_statement(&mut self) -> Result<CompoundStatement, ()> {
        let start = (self.match_and_consume(TokenType::Lbrace, true)?).start_index;
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
        while !self.match_token(TokenType::Rbrace) {
            statement_list.push(self.parse_statement()?);
        }
        let end = (self.match_and_consume(TokenType::Rbrace, true)?).end_index;
        Ok(CompoundStatement {
            local_declaration,
            statement_list,
            start,
            end,
        })
    }
    fn parse_statement(&mut self) -> Result<Statement, ()> {
        match self.next_token() {
            Some(token) => match token.token_type {
                TokenType::Lbrace => Ok(Statement::CompoundStatement(
                    self.parse_compound_statement()?,
                )),
                TokenType::Keyword(KeywordType::IF) => Ok(Statement::SelectionStatement(
                    self.parse_selection_statement()?,
                )),
                TokenType::Keyword(KeywordType::WHILE) => Ok(Statement::IterationStatement(
                    self.parse_iteration_statement()?,
                )),
                TokenType::Keyword(KeywordType::RETURN) => {
                    Ok(Statement::ReturnStatement(self.parse_return_statement()?))
                }
                _ => Ok(self.parse_expression_statement()?),
            },
            None => {
                // return Err(ParseError::from("expected ``"));
                return Err(());
            }
        }
    }
    fn parse_iteration_statement(&mut self) -> Result<IterationStatement, ()> {
        let start =
            (self.match_and_consume(TokenType::Keyword(KeywordType::WHILE), true)?).start_index;
        let expression = self.parse_expression()?;
        let body = Box::new(self.parse_statement()?);
        Ok(IterationStatement {
            start,
            end: body.end(),
            test: expression,
            body,
        })
    }
    fn parse_selection_statement(&mut self) -> Result<SelectionStatement, ()> {
        let start =
            (self.match_and_consume(TokenType::Keyword(KeywordType::IF), true)?).start_index;
        self.match_and_consume(TokenType::Lparen, true)?;
        let test = self.parse_expression()?;
        self.match_and_consume(TokenType::Rparen, true)?;
        let consequent = Box::new(self.parse_statement()?);
        let mut end = consequent.end();
        let alternative = if self.match_token(TokenType::Keyword(KeywordType::ELSE)) {
            self.consume(1);
            let statement = self.parse_statement()?;
            end = statement.end();
            Some(Box::new(statement))
        } else {
            None
        };
        Ok(SelectionStatement {
            consequent,
            alternative,
            test,
            start,
            end,
        })
    }
    fn parse_return_statement(&mut self) -> Result<ReturnStatement, ()> {
        let start =
            (self.match_and_consume(TokenType::Keyword(KeywordType::RETURN), true)?).start_index;
        let mut expression = None;
        if !self.match_token(TokenType::Semi) {
            expression = Some(self.parse_expression()?);
        }
        let end = (self.match_and_consume(TokenType::Semi, true)?).end_index;
        Ok(ReturnStatement {
            expression,
            start,
            end,
        })
    }
    fn parse_expression_statement(&mut self) -> Result<Statement, ()> {
        let mut expression = None;
        let (mut start, mut end) = (0, 0);
        if !self.match_token(TokenType::Semi) {
            let expr = self.parse_expression()?;
            start = expr.start();
            end = expr.end();
            expression = Some(expr);
        }
        let semi_token = self.match_and_consume(TokenType::Semi, true)?;
        if expression.is_none() {
            start = semi_token.start_index;
            end = semi_token.end_index;
        }
        Ok(Statement::ExpressionStatement(ExpressionStatement {
            expression,
            start,
            end,
        }))
    }

    fn parse_var(&mut self) -> Result<Var, ()> {
        let id = self.match_and_consume(TokenType::Id, true)?;
        let mut expression = None;
        let end = if self.match_token(TokenType::Lbrack) {
            self.consume(1);
            expression = Some(Box::new(self.parse_expression()?));
            (self.match_and_consume(TokenType::Rbrack, true)?).end_index
        } else {
            id.end_index
        };
        Ok(Var {
            expression,
            id: Identifier {
                value: id.content,
                start: id.start_index,
                end: id.end_index,
            },
            start: id.start_index,
            end,
        })
    }
    fn parse_assignment_expression(&mut self) -> Result<Expression, ()> {
        let var = self.parse_var()?;
        self.match_and_consume(TokenType::Assign, true)?;
        let expression = self.parse_expression()?;
        Ok(Expression::Assignment(AssignmentExpression {
            start: var.start,
            end: expression.end(),
            lhs: var,
            rhs: Box::new(expression),
        }))
    }
    fn parse_expression(&mut self) -> Result<Expression, ()> {
        let cursor = self.cursor;
        if let Ok(expr) = self.parse_assignment_expression() {
            return Ok(expr);
        }
        self.error_reporter.pop_diagnostic("main.cm");
        // println!("parse_expression: {}", self.error_reporter.emit_string());
        self.cursor = cursor;
        if let Ok(expr) = self.parse_simple_expression() {
            Ok(expr)
        } else {
            // println!("parse_expression: {}", self.error_reporter.emit_string());
            if cursor < self.token_list.len() {
                self.error_reporter.add_diagnostic(
                    "main.cm",
                    self.token_list[cursor].range(),
                    format!(
                        "expected `(`, `identifier`, `number`, found {:?}",
                        self.token_list[cursor].token_type
                    ),
                );
            } else {
                self.error_reporter.add_diagnostic(
                    "main.cm",
                    self.get_source_file_end_range(),
                    "".into(),
                );
            }
            Err(())
        }
    }

    fn parse_simple_expression(&mut self) -> Result<Expression, ()> {
        let left_expr = self.parse_additive_expression()?;
        if let Some(op) = self.match_rel_op() {
            self.consume(1);
            let right_expr = self.parse_additive_expression()?;
            return Ok(Expression::BinaryExpression(BinaryExpression {
                start: left_expr.start(),
                end: right_expr.end(),
                left: Box::new(left_expr),
                right: Box::new(right_expr),
                operation: op,
            }));
        }
        Ok(left_expr)
    }

    fn parse_additive_expression(&mut self) -> Result<Expression, ()> {
        let mut left_term = self.parse_term()?;
        // println!("{:?}", left_term);
        if let Some(_) = self.match_add_op() {
            while let Some(operation) = self.match_add_op() {
                self.consume(1);
                let right_term = self.parse_term()?;
                left_term = Expression::BinaryExpression(BinaryExpression {
                    start: left_term.start(),
                    end: right_term.end(),
                    left: Box::new(left_term),
                    right: Box::new(right_term),
                    operation,
                });
            }
        }
        Ok(left_term)
    }

    fn parse_term(&mut self) -> Result<Expression, ()> {
        let mut left_factor = self.parse_factor()?;
        if let Some(_) = self.match_mul_op() {
            while let Some(operation) = self.match_mul_op() {
                self.consume(1);
                let right_factor = self.parse_factor()?;
                left_factor = Expression::BinaryExpression(BinaryExpression {
                    start: left_factor.start(),
                    end: right_factor.end(),
                    left: Box::new(left_factor),
                    right: Box::new(right_factor),
                    operation,
                });
            }
        }
        Ok(left_factor)
    }

    fn parse_factor(&mut self) -> Result<Expression, ()> {
        if let Some(token) = self.next_token() {
            let content = token.content.clone();
            let range = token.range();
            let mut start = token.start_index;
            let mut end = token.end_index;
            match token.token_type {
                TokenType::NumberLiteral => {
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
                        start,
                        end,
                    })));
                }
                TokenType::BooleanLiteral => {
                    self.consume(1);
                    return Ok(Expression::Factor(Factor::BooleanLiteral(BooleanLiteral {
                        value: content.parse::<bool>().unwrap(),
                        start,
                        end,
                    })));
                }
                TokenType::Lparen => {
                    self.consume(1);
                    let expression = self.parse_expression()?;
                    self.match_and_consume(TokenType::Rparen, true)?;
                    return Ok(Expression::Factor(Factor::Expression(Box::new(expression))));
                }
                TokenType::Id => {
                    let value = token.content.clone();
                    self.consume(1);
                    if let Some(token) = self.next_token() {
                        match token.token_type {
                            TokenType::Lparen => {
                                self.consume(1);
                                let arguments = self.parse_args()?;
                                end = (self.match_and_consume(TokenType::Rparen, true)?).end_index;
                                return Ok(Expression::Factor(Factor::CallExpression(
                                    CallExpression {
                                        arguments,
                                        id: Identifier { value, start, end },
                                        start,
                                        end,
                                    },
                                )));
                            }
                            TokenType::Lbrack => {
                                self.consume(1);
                                let local_expression = self.parse_expression()?;
                                end = (self.match_and_consume(TokenType::Rbrack, true)?).end_index;
                                let var = Var {
                                    id: Identifier { value, start, end },
                                    expression: Some(Box::new(local_expression)),
                                    start,
                                    end,
                                };
                                return Ok(Expression::Factor(Factor::Var(var)));
                            }
                            _ => {
                                return Ok(Expression::Factor(Factor::Var(Var {
                                    expression: None,
                                    id: Identifier { value, start, end },
                                    start,
                                    end,
                                })));
                            }
                        }
                    } else {
                        return Ok(Expression::Factor(Factor::Var(Var {
                            expression: None,
                            id: Identifier { value, start, end },
                            start,
                            end,
                        })));
                    }
                }
                _ => {
                    let token = token.clone();
                    self.error_reporter.add_diagnostic(
                        "main.cm",
                        token.range(),
                        "expected `Identifier`, `NumberLiteral`, `(`".to_string(),
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
        if !self.match_token(TokenType::Rparen) {
            args.push(self.parse_expression()?);
        }
        while !self.match_token(TokenType::Rparen) {
            self.match_and_consume(TokenType::Comma, true)?;
            args.push(self.parse_expression()?);
        }
        Ok(args)
    }

    fn parse_param(&mut self) -> Result<Parameter, ()> {
        let type_specifier = self.parse_type_specifier()?;
        let id_token = self.match_and_consume(TokenType::Id, true)?;
        let identifier = Identifier {
            value: id_token.content,
            start: id_token.start_index,
            end: id_token.end_index,
        };
        let mut is_array = false;
        let end = if self.match_token(TokenType::Lbrack) {
            let end: usize;
            self.match_and_consume(TokenType::Lbrack, true)?;
            end = (self.match_and_consume(TokenType::Rbrack, true)?).end_index;
            is_array = true;
            end
        } else {
            type_specifier.end
        };
        Ok(Parameter {
            start: type_specifier.start,
            end,
            type_specifier,
            id: identifier,
            is_array,
        })
    }
    fn parse_type_specifier(&mut self) -> Result<TypeSpecifier, ()> {
        if let Some(token) = self.next_token() {
            let start = token.start_index;
            let end = token.end_index;
            match token.token_type {
                TokenType::Keyword(KeywordType::INT) => {
                    self.consume(1);
                    return Ok(TypeSpecifier {
                        kind: TypeSpecifierKind::Int,
                        start,
                        end,
                    });
                }
                TokenType::Keyword(KeywordType::VOID) => {
                    self.consume(1);
                    return Ok(TypeSpecifier {
                        kind: TypeSpecifierKind::Void,
                        start,
                        end,
                    });
                }
                TokenType::Keyword(KeywordType::BOOL) => {
                    self.consume(1);
                    return Ok(TypeSpecifier {
                        kind: TypeSpecifierKind::Boolean,
                        start,
                        end,
                    });
                }
                _ => {
                    let token = token.clone();
                    self.error_reporter.add_diagnostic(
                        "main.cm",
                        token.range(),
                        format!(
                            "expected `int` or `void` or `bool`, found {}",
                            token.token_type
                        ),
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
