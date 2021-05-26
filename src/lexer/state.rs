#[derive(PartialEq)]
pub enum State {
    START,
    InDivide,
    // INMULTPLY,
    InNum,
    InId,
    Done,
    InLess,
    InGreat,
    InAssign,
    InNotEqual,
    InComment,
    InEndComment,
    InAnd,
    InOr
}
