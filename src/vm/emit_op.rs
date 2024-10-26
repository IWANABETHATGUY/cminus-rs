use super::error::Error::*;
use super::op_code::OpCode::*;
use super::vm::Vm;
use crate::parser::{ast::*, Codespan};
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
            vm.add_instruction(Nil, start..end);
        }
        vm.define_variable(name.clone(), start..end)?;
        if vm.scope_depth() == 0 {
            vm.add_instruction(Pop, 10000000..10000000);
        }
        Ok(())
    }
}

impl EmitOperationCode for Statement {
    fn emit(&mut self, vm: &mut Vm) -> anyhow::Result<()> {
        use Statement::*;
        match self {
            CompoundStatement(stmt) => {
                stmt.emit(vm)?;
            }
            ExpressionStatement(stmt) => {
                stmt.emit(vm)?;
            }
            SelectionStatement(stmt) => {
                stmt.emit(vm)?;
            }
            IterationStatement(stmt) => {
                stmt.emit(vm)?;
            }
            ReturnStatement(_) => todo!(),
        }
        Ok(())
    }
}
impl EmitOperationCode for IterationStatement {
    fn emit(&mut self, vm: &mut Vm) -> anyhow::Result<()> {
        let loop_start = vm.operations().len();
        self.test.emit(vm)?;
        let exit_jump = vm.emit_jump(JumpIfFalse(0), self.start..self.end);
        vm.add_instruction(Pop, self.test.start()..self.test.end());
        self.body.emit(vm)?;
        vm.emit_loop(loop_start, self.body.start()..self.body.end());
        vm.patch_jump(exit_jump)?;
        Ok(())
    }
}

impl EmitOperationCode for ExpressionStatement {
    fn emit(&mut self, vm: &mut Vm) -> anyhow::Result<()> {
        if let Some(ref mut expr) = self.expression {
            expr.emit(vm)?;
        }
        Ok(())
    }
}
impl EmitOperationCode for SelectionStatement {
    fn emit(&mut self, vm: &mut Vm) -> anyhow::Result<()> {
        self.test.emit(vm)?;
        let then_jump = vm.emit_jump(JumpIfFalse(0), self.start..self.end);
        vm.add_instruction(Pop, self.end..self.end);
        self.consequent.emit(vm)?;
        let else_jump = vm.emit_jump(Jump(0), self.start..self.end);
        vm.patch_jump(then_jump)?;
        vm.add_instruction(Pop, self.end..self.end);
        if let Some(ref mut alternative) = self.alternative {
            alternative.emit(vm)?;
        }
        vm.patch_else_jump(else_jump)?;
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
            Expression::Assignment(assign) => {
                assign.rhs.emit(vm)?;
                let lhs = &assign.lhs;
                if let Some(index) = vm.resolve_local(&lhs.id.value) {
                    vm.add_instruction(SetLocal(index), lhs.id.start..lhs.id.end);
                } else {
                    unreachable!();
                    // vm.add_operation(GetGlobal(var.id.value.clone()), var.start..var.end);
                }
            }
            Expression::BinaryExpression(expr) => {
                expr.left.emit(vm)?;
                expr.right.emit(vm)?;
                match expr.operation {
                    Operation::GT(s, e) => {
                        vm.add_instruction(Greater, s..e);
                    }
                    Operation::LT(s, e) => {
                        vm.add_instruction(Less, s..e);
                    }
                    Operation::GE(s, e) => {
                        vm.add_instruction(GreaterEqual, s..e);
                    }
                    Operation::LE(s, e) => {
                        vm.add_instruction(LessEqual, s..e);
                    }
                    Operation::EQ(s, e) => {
                        vm.add_instruction(Equal, s..e);
                    }
                    Operation::NE(s, e) => {
                        vm.add_instruction(NotEqual, s..e);
                    }
                    Operation::PLUS(s, e) => {
                        vm.add_instruction(AddI32, s..e);
                    }
                    Operation::MINUS(s, e) => {
                        vm.add_instruction(SubtractI32, s..e);
                    }
                    Operation::MULTIPLY(s, e) => {
                        vm.add_instruction(MultiplyI32, s..e);
                    }
                    Operation::DIVIDE(s, e) => {
                        vm.add_instruction(DivideI32, s..e);
                    }
                    _ => unreachable!(),
                }
            }
            Expression::LogicExpression(expr) => {
                expr.left.emit(vm)?;
                expr.right.emit(vm)?;
                match expr.operation {
                    Operation::AND(s, e) => {
                        vm.add_instruction(And, s..e);
                    }
                    Operation::OR(s, e) => {
                        vm.add_instruction(Or, s..e);
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
                        vm.add_instruction(Neg, s..e);
                    }
                    Operation::POS(s, e) => {
                        vm.add_instruction(Pos, s..e);
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
                        vm.add_instruction(GetLocal(index), var.start..var.end);
                    } else {
                        vm.add_instruction(GetGlobal(var.id.value.clone()), var.start..var.end);
                    }
                }
                Factor::CallExpression(_) => todo!(),
                Factor::NumberLiteral(NumberLiteral { value, start, end }) => {
                    vm.add_instruction(ConstantI32(*value), *start..*end);
                }
                Factor::BooleanLiteral(BooleanLiteral { value, start, end }) => {
                    vm.add_instruction(ConstantBoolean(*value), *start..*end);
                }
            },
        }
        Ok(())
    }
}
// impl EmitOperationCode for Expression {

// }
