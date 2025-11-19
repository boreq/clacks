use crate::errors::Error::Unknown;
use anyhow::anyhow;
use chrono::RoundingError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("queue is full")]
    QueueIsFull,

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<RoundingError> for Error {
    fn from(value: RoundingError) -> Self {
        Unknown(anyhow!(value))
    }
}

impl From<chrono::ParseError> for Error {
    fn from(value: chrono::ParseError) -> Self {
        Unknown(anyhow!(value))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
