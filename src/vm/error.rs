use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("Error occurred exec vm, caused by `{0}`")]
    RuntimeError(String),
}
