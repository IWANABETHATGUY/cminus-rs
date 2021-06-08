#[cfg(test)]
mod vm_calc {
    use tinylang_rs::vm::{op_code::OpCode, value::Value, vm::Vm};
    #[test]
    fn test_subtract_i32() {
        // 20 - 10
        let mut vm = Vm::new();
        vm.add_operation(OpCode::ConstantI32(20), 0);
        vm.add_operation(OpCode::ConstantI32(10), 0);
        vm.add_operation(OpCode::SubtractI32, 0);
        vm.exec();
        let result = vec![Value::I32(10)];
        assert_eq!(vm.stack(), &result);
    }
    #[test]
    fn test_add_i32() {
        // 20 + 10
        let mut vm = Vm::new();
        vm.add_operation(OpCode::ConstantI32(20), 0);
        vm.add_operation(OpCode::ConstantI32(10), 0);
        vm.add_operation(OpCode::AddI32, 0);
        vm.exec();
        let result = vec![Value::I32(30)];
        assert_eq!(vm.stack(), &result);
    }
    #[test]
    fn test_divide_i32() {
        // 20 / 10
        let mut vm = Vm::new();
        vm.add_operation(OpCode::ConstantI32(20), 0);
        vm.add_operation(OpCode::ConstantI32(10), 0);
        vm.add_operation(OpCode::DivideI32, 0);
        vm.exec();
        let result = vec![Value::I32(2)];
        assert_eq!(vm.stack(), &result);
    }

    #[test]
    fn test_multiply_i32() {
        // 20 * 10
        let mut vm = Vm::new();
        vm.add_operation(OpCode::ConstantI32(20), 0);
        vm.add_operation(OpCode::ConstantI32(10), 0);
        vm.add_operation(OpCode::MultiplyI32, 0);
        vm.exec();
        let result = vec![Value::I32(200)];
        assert_eq!(vm.stack(), &result);
    }

    #[test]
    fn test_expression_i32() {
        // (2 + 4) / 3
        let mut vm = Vm::new();
        vm.add_operation(OpCode::ConstantI32(2), 0);
        vm.add_operation(OpCode::ConstantI32(4), 0);
        vm.add_operation(OpCode::AddI32, 0);
        vm.add_operation(OpCode::ConstantI32(3), 0);
        vm.add_operation(OpCode::DivideI32, 0);
        vm.exec();
        assert_eq!(vm.stack(), &vec![Value::I32(2)]);
        
        // 1 * 2 + 3
        let mut vm = Vm::new();
        vm.add_operation(OpCode::ConstantI32(1), 0);
        vm.add_operation(OpCode::ConstantI32(2), 0);
        vm.add_operation(OpCode::MultiplyI32, 0);
        vm.add_operation(OpCode::ConstantI32(3), 0);
        vm.add_operation(OpCode::AddI32, 0);
        vm.exec();
        assert_eq!(vm.stack(), &vec![Value::I32(5)]);
        
        // 1 + 2 * 3
        let mut vm = Vm::new();
        vm.add_operation(OpCode::ConstantI32(1), 0);
        vm.add_operation(OpCode::ConstantI32(2), 0);
        vm.add_operation(OpCode::ConstantI32(3), 0);
        vm.add_operation(OpCode::MultiplyI32, 0);
        vm.add_operation(OpCode::AddI32, 0);
        vm.exec();
        assert_eq!(vm.stack(), &vec![Value::I32(7)]);
        
        // 3 - 2 - 1
        let mut vm = Vm::new();
        vm.add_operation(OpCode::ConstantI32(3), 0);
        vm.add_operation(OpCode::ConstantI32(2), 0);
        vm.add_operation(OpCode::SubtractI32, 0);
        vm.add_operation(OpCode::ConstantI32(1), 0);
        vm.add_operation(OpCode::SubtractI32, 0);
        vm.exec();
        assert_eq!(vm.stack(), &vec![Value::I32(0)]);

        let mut vm = Vm::new();
        vm.add_operation(OpCode::ConstantI32(1), 0);
        vm.add_operation(OpCode::ConstantI32(2), 0);
        vm.add_operation(OpCode::ConstantI32(3), 0);
        vm.add_operation(OpCode::MultiplyI32, 0);
        vm.add_operation(OpCode::ConstantI32(4), 0);
        vm.add_operation(OpCode::ConstantI32(-5), 0);
        vm.add_operation(OpCode::DivideI32, 0);
        vm.add_operation(OpCode::SubtractI32, 0);
        vm.add_operation(OpCode::AddI32, 0);
        vm.exec();
        assert_eq!(vm.stack(), &vec![Value::I32(7)]);
    }

}
