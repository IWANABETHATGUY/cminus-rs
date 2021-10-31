use crate::visitor;


struct TacCodeGen {
    
}

impl visitor::Visitor<()> for TacCodeGen {
    fn visit_program(&mut self, node: &crate::parser::ast::Program) -> () {
    }

    fn visit_function_declaration(&mut self, node: &crate::parser::ast::FunctionDeclaration) -> () {
        todo!()
    }

    fn visit_var_declaration(&mut self, node: &crate::parser::ast::VarDeclaration) -> () {
        todo!()
    }

    fn visit_declaration(&mut self, node: &crate::parser::ast::Declaration) -> () {
        todo!()
    }

    fn visit_identifier(&mut self, node: &crate::parser::ast::Identifier) -> () {
        todo!()
    }

    fn visit_number_literal(&mut self, node: &crate::parser::ast::NumberLiteral) -> () {
        todo!()
    }

    fn visit_boolean_literal(&mut self, node: &crate::parser::ast::BooleanLiteral) -> () {
        todo!()
    }

    fn visit_type_specifier(&mut self, node: &crate::parser::ast::TypeSpecifier) -> () {
        todo!()
    }

    fn visit_params(&mut self, node: &crate::parser::ast::Params) -> () {
        todo!()
    }

    fn visit_parameter(&mut self, node: &crate::parser::ast::Parameter) -> () {
        todo!()
    }

    fn visit_compound_statement(&mut self, node: &crate::parser::ast::CompoundStatement) -> () {
        todo!()
    }

    fn visit_statement(&mut self, node: &crate::parser::ast::Statement) -> () {
        todo!()
    }

    fn visit_selection_statement(&mut self, node: &crate::parser::ast::SelectionStatement) -> () {
        todo!()
    }

    fn visit_iteration_statement(&mut self, node: &crate::parser::ast::IterationStatement) -> () {
        todo!()
    }

    fn visit_return_statement(&mut self, node: &crate::parser::ast::ReturnStatement) -> () {
        todo!()
    }

    fn visit_expression_statement(&mut self, node: &crate::parser::ast::ExpressionStatement) -> () {
        todo!()
    }

    fn visit_expression(&mut self, node: &crate::parser::ast::Expression) -> () {
        todo!()
    }

    fn visit_assignment_expression(&mut self, node: &crate::parser::ast::AssignmentExpression) -> () {
        todo!()
    }

    fn visit_var(&mut self, node: &crate::parser::ast::Var) -> () {
        todo!()
    }

    fn visit_logic_expression(&mut self, node: &crate::parser::ast::LogicExpression) -> () {
        todo!()
    }

    fn visit_unary_expression(&mut self, node: &crate::parser::ast::UnaryExpression) -> () {
        todo!()
    }

    fn visit_binary_expression(&mut self, node: &crate::parser::ast::BinaryExpression) -> () {
        todo!()
    }

    fn visit_operation(&mut self, node: &crate::parser::ast::Operation) -> () {
        todo!()
    }

    fn visit_factor(&mut self, node: &crate::parser::ast::Factor) -> () {
        todo!()
    }

    fn visit_call_expression(&mut self, node: &crate::parser::ast::CallExpression) -> () {
        todo!()
    }
}