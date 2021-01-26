use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmulatorError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error("Invalid ELF file: {0}")]
    ElfFormatError(String),
}

pub type EmulatorResult<R> = Result<R, EmulatorError>;
