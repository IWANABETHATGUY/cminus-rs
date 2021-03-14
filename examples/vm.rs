use tinylang_rs::vm::op_code::*;
fn main() {
    let mut vm = Vm::new();
    vm.add_operation(OpCode::ConstantI32(20), 0);
    vm.add_operation(OpCode::ConstantI32(10), 0);
    vm.add_operation(OpCode::SubtractI32, 0);
    vm.add_operation(OpCode::Return, 0);
    vm.exec();
}