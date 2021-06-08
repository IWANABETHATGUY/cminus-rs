#[macro_export]
macro_rules! exp_value {
    ($type:ident) => {
        $type.stack.pop().ok_or(Error::RuntimeError("expected peek of stack is a value"))?
    };
}
