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
                return Err(
                    RuntimeError("last declaration should be function called 'main'").into(),
                );
            }
        } else {
            return Err(RuntimeError("last declaration should be function").into());
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
            Declaration::FunctionDeclaration(_) => {}
            Declaration::VarDeclaration(var_decl) => {
                let name = &var_decl.id.value;
                let (start, end) = (var_decl.start, var_decl.end);
                if let Some(ref mut init) = var_decl.initializer {
                    init.emit(vm)?;
                } else {
                    vm.add_operation(Nil, start..end);
                }
                vm.define_variable(name.clone(), start..end);
                vm.add_operation(Pop, end..end);
            }
        }
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
            Expression::Assignment(_) => todo!(),
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
                    Operation::AND(_, _) => todo!(),
                    Operation::OR(_, _) => todo!(),
                    Operation::NEG(_, _) => todo!(),
                    Operation::POS(_, _) => todo!(),
                }
            }
            Expression::LogicExpression(_) => todo!(),
            Expression::UnaryExpression(_) => todo!(),
            Expression::Factor(expr) => match expr {
                Factor::Expression(expr) => {
                    expr.emit(vm)?;
                }
                Factor::Var(_) => todo!(),
                Factor::CallExpression(_) => todo!(),
                Factor::NumberLiteral(NumberLiteral { value, start, end }) => {
                    vm.add_operation(ConstantI32(*value), *start..*end);
                }
                Factor::BooleanLiteral(_) => todo!(),
            },
        }
        Ok(())
    }
}
// impl EmitOperationCode for Expression {

// }
