use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Given path is not a gci file.")]
    NotGciFile
}