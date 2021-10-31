use super::{ast::*, Codespan};

pub trait Visitor<T> {
    fn visit_program(&mut self, node: &Program) -> T;
    fn visit_function_declaration(&mut self, node: &FunctionDeclaration) -> T;
    fn visit_var_declaration(&mut self, node: &VarDeclaration) -> T;
    fn visit_declaration(&mut self, node: &Declaration) -> T;
    fn visit_identifier(&mut self, node: &Identifier) -> T;
    fn visit_number_literal(&mut self, node: &NumberLiteral) -> T;
    fn visit_boolean_literal(&mut self, node: &BooleanLiteral) -> T;
    fn visit_type_specifier(&mut self, node: &TypeSpecifier) -> T;
    fn visit_params(&mut self, node: &Params) -> T;
    fn visit_parameter(&mut self, node: &Parameter) -> T;
    fn visit_compound_statement(&mut self, node: &CompoundStatement) -> T;
    fn visit_statement(&mut self, node: &Statement) -> T;
    fn visit_selection_statement(&mut self, node: &SelectionStatement) -> T;
    fn visit_iteration_statement(&mut self, node: &IterationStatement) -> T;
    fn visit_return_statement(&mut self, node: &ReturnStatement) -> T;
    fn visit_expression_statement(&mut self, node: &ExpressionStatement) -> T;
    fn visit_expression(&mut self, node: &Expression) -> T;
    fn visit_assignment_expression(&mut self, node: &AssignmentExpression) -> T;
    fn visit_var(&mut self, node: &Var) -> T;
    fn visit_logic_expression(&mut self, node: &LogicExpression) -> T;
    fn visit_unary_expression(&mut self, node: &UnaryExpression) -> T;
    fn visit_binary_expression(&mut self, node: &BinaryExpression) -> T;
    fn visit_operation(&mut self, node: &Operation) -> T;
    fn visit_factor(&mut self, node: &Factor) -> T;
    fn visit_call_expression(&mut self, node: &CallExpression) -> T;
}

#[derive(Default)]
pub struct AstPrinter {
    level: usize,
}

impl AstPrinter {
    pub fn print_ast(&mut self, program: &Program) -> String {
        self.visit_program(program)
    }
}


impl Visitor<String> for AstPrinter {
    fn visit_program(&mut self, node: &Program) -> String {
        let ast = format!(
            "{}Program {}\n",
            " ".repeat(2 * self.level),
            generate_codespan_postfix(node)
        );
        let mut children = vec![];
        self.level += 1;
        for decl in node.declarations.iter() {
            children.push(self.visit_declaration(decl));
        }
        self.level -= 1;
        ast + &children.join("\n")
    }

    fn visit_function_declaration(&mut self, node: &FunctionDeclaration) -> String {
        let mut ast = format!(
            "{}FunctionDeclaration {}\n",
            " ".repeat(2 * self.level),
            generate_codespan_postfix(node)
        );
        self.level += 1;
        ast += &vec![
            self.visit_type_specifier(&node.type_specifier),
            self.visit_identifier(&node.id),
            self.visit_params(&node.params),
            self.visit_compound_statement(&node.body),
        ]
        .join("\n");
        self.level -= 1;
        ast
    }

    fn visit_var_declaration(&mut self, node: &VarDeclaration) -> String {
        let ast = format!(
            "{}VarDeclaration {}\n",
            " ".repeat(2 * self.level),
            generate_codespan_postfix(node)
        );
        self.level += 1;
        let mut children = vec![
            self.visit_type_specifier(&node.type_specifier),
            self.visit_identifier(&node.id),
        ];
        if let Some(ref num) = node.num {
            children.push(self.visit_number_literal(num));
        }
        if let Some(ref initializer) = node.initializer {
            children.push(
                format!("{}<Initializer>", " ".repeat(2 * (self.level + 1)),)
                    + &self.visit_expression(initializer).trim_start(),
            );
        }
        if let Some(ref initializer) = node.array_initializer {
            children.push(format!("{}<Initializer>", " ".repeat(2 * (self.level + 1)),));
            self.level += 1;
            for init in initializer {
                children.push(self.visit_expression(init));
            }
            self.level -= 1;
        }
        self.level -= 1;
        ast + &children.join("\n")
    }

    fn visit_declaration(&mut self, node: &Declaration) -> String {
        match node {
            Declaration::VarDeclaration(var_decl) => self.visit_var_declaration(var_decl),
            Declaration::FunctionDeclaration(func_decl) => {
                self.visit_function_declaration(func_decl)
            }
        }
    }

    fn visit_identifier(&mut self, node: &Identifier) -> String {
        format!(
            "{}Identifier({}) {}",
            " ".repeat(2 * self.level),
            node.value,
            generate_codespan_postfix(node)
        )
    }

    fn visit_number_literal(&mut self, node: &NumberLiteral) -> String {
        format!(
            "{}NumberLiteral({}) {}",
            " ".repeat(2 * self.level),
            node.value,
            generate_codespan_postfix(node)
        )
    }

    fn visit_boolean_literal(&mut self, node: &BooleanLiteral) -> String {
        format!(
            "{}BooleanLiteral({}) {}",
            " ".repeat(2 * self.level),
            node.value,
            generate_codespan_postfix(node)
        )
    }

    fn visit_type_specifier(&mut self, node: &TypeSpecifier) -> String {
        format!(
            "{}TypeSpecifier({:?}) {}",
            " ".repeat(2 * self.level),
            node.kind,
            generate_codespan_postfix(node)
        )
    }

    fn visit_params(&mut self, node: &Params) -> String {
        match node {
            Params::Void => format!(
                "{}Void {}",
                " ".repeat(2 * self.level),
                generate_codespan_postfix(node)
            ),
            Params::ParamsList { params } => {
                let params_codespan = if params.len() > 0 {
                    let start = params[0].start;
                    let end = params[params.len() - 1].end;
                    format!("@{}..{}", start, end)
                } else {
                    "".to_string()
                };
                let ast = format!(
                    "{}ParameterList {}",
                    " ".repeat(2 * self.level),
                    params_codespan
                );
                if !params.is_empty() {
                    self.level += 1;
                    let ret = ast
                        + "\n"
                        + &params
                            .iter()
                            .map(|param| self.visit_parameter(param))
                            .filter(|param| !param.is_empty())
                            .collect::<Vec<String>>()
                            .join("\n");
                    self.level -= 1;
                    ret
                } else {
                    ast
                }
            }
        }
    }

    fn visit_parameter(&mut self, node: &Parameter) -> String {
        format!(
            "{}Parameter({:?} {}{}) {}",
            " ".repeat(2 * self.level),
            node.type_specifier.kind,
            node.id.value,
            if node.is_array { "[]" } else { "" },
            generate_codespan_postfix(node)
        )
    }

    fn visit_compound_statement(&mut self, node: &CompoundStatement) -> String {
        let mut ast = format!(
            "{}CompoundStatement {}",
            " ".repeat(2 * self.level),
            generate_codespan_postfix(node)
        );
        if !node.local_declaration.is_empty() {
            self.level += 1;
            ast = ast
                + "\n"
                + &node
                    .local_declaration
                    .iter()
                    .map(|decl| self.visit_var_declaration(decl))
                    .filter(|item| !item.is_empty())
                    .collect::<Vec<String>>()
                    .join("\n");
            self.level -= 1;
        }
        if !node.statement_list.is_empty() {
            self.level += 1;
            ast = ast
                + "\n"
                + &node
                    .statement_list
                    .iter()
                    .map(|stmt| self.visit_statement(stmt))
                    .filter(|item| !item.is_empty())
                    .collect::<Vec<String>>()
                    .join("\n");
            self.level -= 1;
        }
        ast
    }

    fn visit_statement(&mut self, node: &Statement) -> String {
        match node {
            Statement::CompoundStatement(stmt) => self.visit_compound_statement(stmt),
            Statement::ExpressionStatement(stmt) => self.visit_expression_statement(stmt),
            Statement::SelectionStatement(stmt) => self.visit_selection_statement(stmt),
            Statement::IterationStatement(stmt) => self.visit_iteration_statement(stmt),
            Statement::ReturnStatement(stmt) => self.visit_return_statement(stmt),
        }
    }

    fn visit_selection_statement(&mut self, node: &SelectionStatement) -> String {
        let ast = format!(
            "{}SelectionStatement {}\n",
            " ".repeat(2 * self.level),
            generate_codespan_postfix(node)
        );
        self.level += 1;
        let mut children = vec![
            self.visit_expression(&node.test),
            self.visit_statement(&node.consequent),
        ];
        if let Some(ref consequent) = node.alternative {
            children.push(self.visit_statement(&consequent));
        }
        self.level -= 1;
        ast + &children
            .into_iter()
            .filter(|child| !child.is_empty())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn visit_iteration_statement(&mut self, node: &IterationStatement) -> String {
        let ast = format!(
            "{}IterationStatement {}\n",
            " ".repeat(2 * self.level),
            generate_codespan_postfix(node)
        );
        self.level += 1;
        let mut children = vec![self.visit_expression(&node.test)];
        let body_ast_string = self.visit_statement(&node.body);
        self.level -= 1;
        if !body_ast_string.is_empty() {
            children.push(body_ast_string);
        }
        ast + &children.join("\n")
    }

    fn visit_return_statement(&mut self, node: &ReturnStatement) -> String {
        let mut ast = format!(
            "{}ReturnStatement {}\n",
            " ".repeat(2 * self.level),
            generate_codespan_postfix(node)
        );
        if let Some(ref expr) = node.expression {
            self.level += 1;
            ast += &self.visit_expression(expr);
            self.level -= 1;
        }
        ast
    }

    fn visit_expression_statement(&mut self, node: &ExpressionStatement) -> String {
        if let Some(ref expr) = node.expression {
            self.visit_expression(expr)
        } else {
            "".to_string()
        }
    }

    fn visit_expression(&mut self, node: &Expression) -> String {
        match node {
            Expression::Assignment(assignment) => {
                let ast = format!(
                    "{}Assignment {}\n",
                    " ".repeat(2 * self.level),
                    generate_codespan_postfix(node)
                );
                ast + &self.visit_assignment_expression(assignment)
            }
            Expression::BinaryExpression(binary_expr) => {
                let ast = format!(
                    "{}BinaryExpression {}\n",
                    " ".repeat(2 * self.level),
                    generate_codespan_postfix(node)
                );
                self.level += 1;
                let children = vec![
                    self.visit_expression(&binary_expr.left),
                    self.visit_operation(&binary_expr.operation),
                    self.visit_expression(&binary_expr.right),
                ];
                self.level -= 1;
                ast + &children.join("\n")
            }
            Expression::Factor(factor) => self.visit_factor(factor),
            Expression::LogicExpression(logic_expr) => {
                let ast = format!(
                    "{}LogicExpression {}\n",
                    " ".repeat(2 * self.level),
                    generate_codespan_postfix(node)
                );
                self.level += 1;
                let children = vec![
                    self.visit_expression(&logic_expr.left),
                    self.visit_operation(&logic_expr.operation),
                    self.visit_expression(&logic_expr.right),
                ];
                self.level -= 1;
                ast + &children.join("\n")
            }
            Expression::UnaryExpression(expr) => {
                let ast = format!(
                    "{}UnaryExpression {}\n",
                    " ".repeat(2 * self.level),
                    generate_codespan_postfix(node)
                );
                self.level += 1;
                let children = vec![
                    self.visit_operation(&expr.operation),
                    self.visit_expression(&expr.expression),
                ];
                self.level -= 1;
                ast + &children.join("\n")
            }
        }
    }

    fn visit_assignment_expression(&mut self, node: &AssignmentExpression) -> String {
        self.level += 1;
        let ret = format!(
            "{}\n{}",
            self.visit_var(&node.lhs),
            self.visit_expression(&node.rhs)
        );
        self.level -= 1;
        ret
    }

    fn visit_var(&mut self, node: &Var) -> String {
        self.level += 1;
        let id = self.visit_identifier(&node.id);
        self.level -= 1;
        let mut result = vec![
            format!(
                "{}Var {}",
                " ".repeat(2 * self.level),
                generate_codespan_postfix(node)
            ),
            id,
        ];
        if let Some(ref expr) = node.expression {
            self.level += 1;
            result.push(self.visit_expression(expr));
            self.level -= 1;
        }
        result.join("\n")
    }

    fn visit_logic_expression(&mut self, node: &LogicExpression) -> String {
        todo!()
    }

    fn visit_unary_expression(&mut self, node: &UnaryExpression) -> String {
        todo!()
    }

    fn visit_binary_expression(&mut self, node: &BinaryExpression) -> String {
        todo!()
    }

    fn visit_operation(&mut self, node: &Operation) -> String {
        format!(
            "{}{} {}",
            " ".repeat(2 * self.level),
            node,
            generate_codespan_postfix(node)
        )
    }

    fn visit_factor(&mut self, node: &Factor) -> String {
        match node {
            Factor::Expression(expr) => self.visit_expression(expr),
            Factor::Var(var) => self.visit_var(var),
            Factor::CallExpression(call) => self.visit_call_expression(call),
            Factor::NumberLiteral(num) => self.visit_number_literal(num),
            Factor::BooleanLiteral(boolean) => self.visit_boolean_literal(boolean),
        }
    }

    fn visit_call_expression(&mut self, node: &CallExpression) -> String {
        let ast = format!(
            "{}CallExpression {}\n",
            " ".repeat(self.level * 2),
            generate_codespan_postfix(node)
        );
        let arguments_codespan = if node.arguments.len() > 0 {
            let start = node.arguments[0].start();
            let end = node.arguments[node.arguments.len() - 1].end();
            format!("@{}..{}", start, end)
        } else {
            "".to_string()
        };
        let mut children = vec![];
        self.level += 1;
        children.push(self.visit_identifier(&node.id));
        self.level -= 1;
        children.push(format!(
            "{}Arguments {}",
            " ".repeat((self.level + 1) * 2),
            arguments_codespan
        ));
        self.level += 2;
        node.arguments
            .iter()
            .map(|arg| self.visit_expression(arg))
            .filter(|arg| !arg.is_empty())
            .collect::<Vec<String>>()
            .join("\n");
        self.level -= 2;
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
