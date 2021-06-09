use super::error::Error::*;
use super::op_code::OpCode::*;
use super::vm::Vm;
use crate::parser::ast::*;
pub trait EmitOperationCode {
    fn emit(&mut self, vm: &mut Vm) -> anyhow::Result<()>;
}

impl EmitOperationCode for Program {
    fn emit(&mut self, vm: &mut Vm) -> anyhow::Result<()> {
        if let Some(Declaration::FunctionDeclaration(func)) = self.declarations.last() {
            if &func.id.value != "main" {
                return Err(RuntimeError(
                    "last declaration should be function called 'main'".to_string(),
                )
                .into());
            }
        } else {
            return Err(RuntimeError("last declaration should be function".to_string()).into());
        }
        for decl in self.declarations.iter_mut() {
            decl.emit(vm)?;
        }
        Ok(())
    }
}

impl EmitOperationCode for Declaration {
    fn emit(&mut self, vm: &mut Vm) -> anyhow::Result<()> {
        match self {
            Declaration::FunctionDeclaration(decl) => {
                // TODO: params
                decl.body.emit(vm)?;
            }
            Declaration::VarDeclaration(var_decl) => {
                var_decl.emit(vm)?;
            }
        }
        Ok(())
    }
}
impl EmitOperationCode for VarDeclaration {
    fn emit(&mut self, vm: &mut Vm) -> anyhow::Result<()> {
        let name = &self.id.value;
        let (start, end) = (self.start, self.end);
        if let Some(ref mut init) = self.initializer {
            init.emit(vm)?;
        } else {
            vm.add_operation(Nil, start..end);
        }
        vm.define_variable(name.clone(), start..end)?;
        if vm.scope_depth() == 0 {
            vm.add_operation(Pop, end..end);
        }
        Ok(())
    }
}

impl EmitOperationCode for Statement {
    fn emit(&mut self, vm: &mut Vm) -> anyhow::Result<()> {
        match self {
            Statement::CompoundStatement(stmt) => {
                stmt.emit(vm)?;
            }
            Statement::ExpressionStatement(stmt) => todo!(),
            Statement::SelectionStatement(_) => todo!(),
            Statement::IterationStatement(_) => todo!(),
            Statement::ReturnStatement(_) => todo!(),
        }
        Ok(())
    }
}

impl EmitOperationCode for CompoundStatement {
    fn emit(&mut self, vm: &mut Vm) -> anyhow::Result<()> {
        vm.begin_scope();
        for decl in self.local_declaration.iter_mut() {
            decl.emit(vm)?;
        }
        for stmt in self.statement_list.iter_mut() {
            stmt.emit(vm)?;
        }
        vm.end_scope();
        Ok(())
    }
}

impl EmitOperationCode for FunctionDeclaration {
    fn emit(&mut self, vm: &mut Vm) -> anyhow::Result<()> {
        // TODO:
        Ok(())
    }
}

impl EmitOperationCode for Expression {
    fn emit(&mut self, vm: &mut Vm) -> anyhow::Result<()> {
        match self {
            Expression::Assignment(assign) => todo!(),
            Expression::BinaryExpression(expr) => {
                expr.left.emit(vm)?;
                expr.right.emit(vm)?;
                match expr.operation {
                    Operation::GT(s, e) => {
                        vm.add_operation(Greater, s..e);
                    }
                    Operation::LT(s, e) => {
                        vm.add_operation(Less, s..e);
                    }
                    Operation::GE(s, e) => {
                        vm.add_operation(GreaterEqual, s..e);
                    }
                    Operation::LE(s, e) => {
                        vm.add_operation(LessEqual, s..e);
                    }
                    Operation::EQ(s, e) => {
                        vm.add_operation(Equal, s..e);
                    }
                    Operation::NE(s, e) => {
                        vm.add_operation(NotEqual, s..e);
                    }
                    Operation::PLUS(s, e) => {
                        vm.add_operation(AddI32, s..e);
                    }
                    Operation::MINUS(s, e) => {
                        vm.add_operation(SubtractI32, s..e);
                    }
                    Operation::MULTIPLY(s, e) => {
                        vm.add_operation(MultiplyI32, s..e);
                    }
                    Operation::DIVIDE(s, e) => {
                        vm.add_operation(DivideI32, s..e);
                    }
                    _ => unreachable!(),
                }
            }
            Expression::LogicExpression(expr) => {
                expr.left.emit(vm)?;
                expr.right.emit(vm)?;
                match expr.operation {
                    Operation::AND(s, e) => {
                        vm.add_operation(And, s..e);
                    }
                    Operation::OR(s, e) => {
                        vm.add_operation(Or, s..e);
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
            Expression::UnaryExpression(expr) => {
                expr.expression.emit(vm)?;
                match expr.operation {
                    Operation::NEG(s, e) => {
                        vm.add_operation(Neg, s..e);
                    }
                    Operation::POS(s, e) => {
                        vm.add_operation(Pos, s..e);
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
            Expression::Factor(expr) => match expr {
                Factor::Expression(expr) => {
                    expr.emit(vm)?;
                }
                Factor::Var(var) => {
                    if let Some(index) = vm.resolve_local(&var.id.value) {
                        vm.add_operation(GetLocal(index), var.start..var.end);
                    } else {
                        vm.add_operation(GetGlobal(var.id.value.clone()), var.start..var.end);
                    }
                }
                Factor::CallExpression(_) => todo!(),
                Factor::NumberLiteral(NumberLiteral { value, start, end }) => {
                    vm.add_operation(ConstantI32(*value), *start..*end);
                }
                Factor::BooleanLiteral(BooleanLiteral { value, start, end }) => {
                    vm.add_operation(ConstantBoolean(*value), *start..*end);
                }
            },
        }
        Ok(())
    }
}
// impl EmitOperationCode for Expression {

// }
