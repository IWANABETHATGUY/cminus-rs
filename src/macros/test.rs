#[macro_export]
macro_rules! P {
    ($line:expr, $column:expr) => {
        Position::new($line, $column)
    };
}

#[macro_export]
macro_rules! T {
    ($type:ident) => {
        TokenType::$type
    };
}
