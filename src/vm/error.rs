use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error<'a> {
    #[error("Error occurred exec vm, caused by `{0}`")]
    RuntimeError(&'a str),
}
