use crate::errors::Error::Unknown;
use anyhow::anyhow;
use chrono::RoundingError;
use clap::parser::MatchesError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("queue is full")]
    QueueIsFull,

    #[error("cannot encode character '{0}'")]
    CannotEncodeCharacter(char),

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

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Unknown(anyhow!(value))
    }
}

impl From<toml::de::Error> for Error {
    fn from(value: toml::de::Error) -> Self {
        Unknown(anyhow!(value))
    }
}

impl From<prometheus::Error> for Error {
    fn from(value: prometheus::Error) -> Self {
        Unknown(anyhow!(value))
    }
}

impl From<MatchesError> for Error {
    fn from(value: MatchesError) -> Self {
        Unknown(anyhow!(value))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
