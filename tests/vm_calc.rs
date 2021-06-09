#[cfg(test)]
mod vm_calc {
    use tinylang_rs::vm::{op_code::OpCode::*, value::Value, Vm};
    #[test]
    fn test_subtract_i32() {
        // 20 - 10
        let mut vm = Vm::new();
        vm.add_operation(ConstantI32(20), 0..0);
        vm.add_operation(ConstantI32(10), 0..0);
        vm.add_operation(SubtractI32, 0..0);
        vm.exec().unwrap();
        let result = vec![Value::I32(10)];
        assert_eq!(vm.stack(), &result);
    }
    #[test]
    fn test_add_i32() {
        // 20 + 10
        let mut vm = Vm::new();
        vm.add_operation(ConstantI32(20), 0..0);
        vm.add_operation(ConstantI32(10), 0..0);
        vm.add_operation(AddI32, 0..0);
        vm.exec().unwrap();
        let result = vec![Value::I32(30)];
        assert_eq!(vm.stack(), &result);
    }
    #[test]
    fn test_divide_i32() {
        // 20 / 10
        let mut vm = Vm::new();
        vm.add_operation(ConstantI32(20), 0..0);
        vm.add_operation(ConstantI32(10), 0..0);
        vm.add_operation(DivideI32, 0..0);
        vm.exec().unwrap();
        let result = vec![Value::I32(2)];
        assert_eq!(vm.stack(), &result);
    }

    #[test]
    fn test_multiply_i32() {
        // 20 * 10
        let mut vm = Vm::new();
        vm.add_operation(ConstantI32(20), 0..0);
        vm.add_operation(ConstantI32(10), 0..0);
        vm.add_operation(MultiplyI32, 0..0);
        vm.exec().unwrap();
        let result = vec![Value::I32(200)];
        assert_eq!(vm.stack(), &result);
    }

    #[test]
    fn test_expression_i32() {
        // (2 + 4) / 3
        let mut vm = Vm::new();
        vm.add_operation(ConstantI32(2), 0..0);
        vm.add_operation(ConstantI32(4), 0..0);
        vm.add_operation(AddI32, 0..0);
        vm.add_operation(ConstantI32(3), 0..0);
        vm.add_operation(DivideI32, 0..0);
        vm.exec().unwrap();
        assert_eq!(vm.stack(), &vec![Value::I32(2)]);

        // 1 * 2 + 3
        let mut vm = Vm::new();
        vm.add_operation(ConstantI32(1), 0..0);
        vm.add_operation(ConstantI32(2), 0..0);
        vm.add_operation(MultiplyI32, 0..0);
        vm.add_operation(ConstantI32(3), 0..0);
        vm.add_operation(AddI32, 0..0);
        vm.exec().unwrap();
        assert_eq!(vm.stack(), &vec![Value::I32(5)]);

        // 1 + 2 * 3
        let mut vm = Vm::new();
        vm.add_operation(ConstantI32(1), 0..0);
        vm.add_operation(ConstantI32(2), 0..0);
        vm.add_operation(ConstantI32(3), 0..0);
        vm.add_operation(MultiplyI32, 0..0);
        vm.add_operation(AddI32, 0..0);
        vm.exec().unwrap();
        assert_eq!(vm.stack(), &vec![Value::I32(7)]);

        // 3 - 2 - 1
        let mut vm = Vm::new();
        vm.add_operation(ConstantI32(3), 0..0);
        vm.add_operation(ConstantI32(2), 0..0);
        vm.add_operation(SubtractI32, 0..0);
        vm.add_operation(ConstantI32(1), 0..0);
        vm.add_operation(SubtractI32, 0..0);
        vm.exec().unwrap();
        assert_eq!(vm.stack(), &vec![Value::I32(0)]);

        // 1 + 2 * 3 - 4 / -5
        let mut vm = Vm::new();
        vm.add_operation(ConstantI32(1), 0..0);
        vm.add_operation(ConstantI32(2), 0..0);
        vm.add_operation(ConstantI32(3), 0..0);
        vm.add_operation(MultiplyI32, 0..0);
        vm.add_operation(AddI32, 0..0);
        vm.add_operation(ConstantI32(4), 0..0);
        vm.add_operation(ConstantI32(-5), 0..0);
        vm.add_operation(DivideI32, 0..0);
        vm.add_operation(SubtractI32, 0..0);
        vm.exec().unwrap();
        assert_eq!(vm.stack(), &vec![Value::I32(7)]);

        // 1 < 2
        let mut vm = Vm::new();
        vm.add_operation(ConstantI32(1), 0..0);
        vm.add_operation(ConstantI32(2), 0..0);
        vm.add_operation(Less, 0..0);
        vm.exec().unwrap();
        assert_eq!(vm.stack(), &vec![Value::Boolean(true)]);

        // 1 + 2 < 3
        let mut vm = Vm::new();
        vm.add_operation(ConstantI32(1), 0..0);
        vm.add_operation(ConstantI32(2), 0..0);
        vm.add_operation(AddI32, 0..0);
        vm.add_operation(ConstantI32(2), 0..0);
        vm.add_operation(Less, 0..0);
        vm.exec().unwrap();
        assert_eq!(vm.stack(), &vec![Value::Boolean(false)]);

        // 1 + 2 <= 3;
        let mut vm = Vm::new();
        vm.add_operation(ConstantI32(1), 0..0);
        vm.add_operation(ConstantI32(2), 0..0);
        vm.add_operation(AddI32, 0..0);
        vm.add_operation(ConstantI32(3), 0..0);
        vm.add_operation(LessEqual, 0..0);
        vm.exec().unwrap();
        assert_eq!(vm.stack(), &vec![Value::Boolean(true)]);
    }

    #[test]
    fn test_logic_expr() {
        // 1 + 2 < 3 && 32 - 1 < 8;
        let mut vm = Vm::new();

        vm.add_operation(ConstantI32(1), 0..0);
        vm.add_operation(ConstantI32(2), 0..0);
        vm.add_operation(AddI32, 0..0);
        vm.add_operation(ConstantI32(3), 0..0);
        vm.add_operation(Less, 0..0);

        vm.add_operation(ConstantI32(32), 0..0);
        vm.add_operation(ConstantI32(1), 0..0);
        vm.add_operation(SubtractI32, 0..0);
        vm.add_operation(ConstantI32(8), 0..0);
        vm.add_operation(Less, 0..0);

        vm.add_operation(And, 0..0);

        vm.exec().unwrap();
        assert_eq!(vm.stack(), &vec![Value::Boolean(false)]);
        // false || true
        let mut vm = Vm::new();

        vm.add_operation(ConstantBoolean(false), 0..0);
        vm.add_operation(ConstantBoolean(true), 0..0);
        vm.add_operation(Or, 0..0);
        vm.exec().unwrap();
        assert_eq!(vm.stack(), &vec![Value::Boolean(true)]);
    }

    #[test]
    fn unary_expression() {
        // -10
        let mut vm = Vm::new();

        vm.add_operation(ConstantI32(10), 0..0);
        vm.add_operation(Neg, 0..0);
        vm.exec().unwrap();
        assert_eq!(vm.stack(), &vec![Value::I32(-10)]);
        // 1 + 2 * 3 - 4 / -2
        let mut vm = Vm::new();
        vm.add_operation(ConstantI32(1), 0..0);
        vm.add_operation(ConstantI32(2), 0..0);
        vm.add_operation(ConstantI32(3), 0..0);
        vm.add_operation(MultiplyI32, 0..0);
        vm.add_operation(AddI32, 0..0);
        vm.add_operation(ConstantI32(4), 0..0);
        vm.add_operation(ConstantI32(2), 0..0);
        vm.add_operation(Neg, 0..0);
        vm.add_operation(DivideI32, 0..0);
        vm.add_operation(SubtractI32, 0..0);
        vm.exec().unwrap();
        assert_eq!(vm.stack(), &vec![Value::I32(9)]);
    }
}
