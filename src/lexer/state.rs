#[derive(PartialEq)]
pub enum State {
    START,
    INDIVIDE,
    // INMULTPLY,
    INNUM,
    INID,
    DONE,
    INLESS,
    INGREAT,
    INASSIGN,
    INNOTEQUAL,
    INCOMMENT,
    INECOMMENT,
}
