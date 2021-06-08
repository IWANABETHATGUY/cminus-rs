use super::error::Error::*;
use super::vm::Vm;
use crate::parser::ast::*;
use super::op_code::OpCode::*;
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
                    vm.define_variable(name.clone(), start..end);
                } else {
                    vm.add_operation(Nil, start..end);
                }
                vm.add_operation(Pop, end..end);
            }
        }
        Ok(())
    }
}

impl EmitOperationCode for FunctionDeclaration {
    fn emit(&mut self, vm: &mut Vm) -> anyhow::Result<()> {
        Ok(())
    }
}

impl EmitOperationCode for Expression {
    fn emit(&mut self, vm: &mut Vm) -> anyhow::Result<()> {
        todo!()
    }
}
// impl EmitOperationCode for Expression {

// }
