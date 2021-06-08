use super::{error::Error::*, vm::Vm, };
use crate::parser::ast::*;
pub trait EmitOperationCode {
    fn emit(&self, vm: &mut Vm) -> anyhow::Result<()>;
}

impl EmitOperationCode for Program {
    fn emit(&self, _vm: &mut Vm) -> anyhow::Result<()> {
        if let Some(Declaration::FunctionDeclaration(func)) = self.declarations.last() {
            if &func.id.value != "main" {
                return Err(
                    RuntimeError("last declaration should be function called 'main'").into(),
                );
            }
        } else {
            return Err(RuntimeError("last declaration should be function").into());
        }
        Ok(())
    }
}

impl EmitOperationCode for Declaration {
    fn emit(&self, _vm: &mut Vm) -> anyhow::Result<()> {
        match self {
            Declaration::FunctionDeclaration(_) => {}
            Declaration::VarDeclaration(_) => {}
        }
        Ok(())
    }
}

impl EmitOperationCode for FunctionDeclaration {
    fn emit(&self, _vm: &mut Vm) -> anyhow::Result<()> {
        Ok(())
    }
}

// impl EmitOperationCode for Expression {

// }