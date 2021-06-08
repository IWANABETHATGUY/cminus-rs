#[cfg(test)]
mod emit_op {
    use std::{fs::read_to_string, path};

    use anyhow::{anyhow, Result};
    use tinylang_rs::{
        lexer::lex,
        parser::parse::Parser,
        vm::{EmitOperationCode, Vm},
    };
    fn get_vm_after_emit(content: &str) -> Result<Vm> {
        let mut lex = lex::Lexer::new(content);
        let list = lex.lex();
        // println!("{:?}", list);
        let mut parser = Parser::new(list, content);
        let mut program = match parser.parse_program() {
            Ok(prog) => prog,
            Err(_) => {
                let string = parser.error_reporter.emit_string();
                return Err(anyhow!(string));
            }
        };
        let mut vm = Vm::new();
        program.emit(&mut vm)?;
        Ok(vm)
    }
    #[test]
    fn test_global_var() -> Result<()> {
        use tinylang_rs::vm::op_code::OpCode::*;

        let path = path::Path::new("tests/fixtures/vm/global.cm");
        let content = read_to_string(path)?;
        let vm = get_vm_after_emit(&content)?;
        assert_eq!(
            vm.operations(),
            &vec![
                ConstantI32(3),
                ConstantI32(2),
                AddI32,
                ConstantI32(4),
                AddI32,
                DefineGlobal("a".into()),
                Pop
            ]
        );
        Ok(())
    }
    #[test]
    fn test_expression() -> Result<()> {
        use tinylang_rs::vm::op_code::OpCode::*;

        let content = r#"
        int a = 1 + 2 * 3;
        void main() {}
        "#;
        let vm = get_vm_after_emit(&content)?;
        assert_eq!(
            vm.operations(),
            &vec![
                ConstantI32(1),
                ConstantI32(2),
                ConstantI32(3),
                MultiplyI32,
                AddI32,
                DefineGlobal("a".into()),
                Pop
            ]
        );

        let content = r#"
        int a = 1 + 2 * 3 - 4 / 5;
        void main() {}
        "#;
        let vm = get_vm_after_emit(&content)?;
        assert_eq!(
            vm.operations(),
            &vec![
                ConstantI32(1),
                ConstantI32(2),
                ConstantI32(3),
                MultiplyI32,
                AddI32,
                ConstantI32(4),
                ConstantI32(5),
                DivideI32,
                SubtractI32,
                DefineGlobal("a".into()),
                Pop
            ]
        );

        Ok(())
    }
}
