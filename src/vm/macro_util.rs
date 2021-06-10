#[macro_export]
macro_rules! expect_value {
    ($type:ident) => {
        $type.stack.pop().ok_or(Error::RuntimeError(
            "expected peek of stack is a value".to_string(),
        ))?
    };
}

#[macro_export]
macro_rules! trace {
    ($type:ident, $op:ident) => {{
        // DEBUG: start
        if cfg!(debug_assertions) {
            println!("-------start--------");
            disassemble_instruction($op, $type.line_number[$type.ip].clone());
            println!("stack: {:?}", $type.stack);
            println!("locals: {:?}", $type.compiler.locals);
        }
        // DEBUG: end
    }};
}
