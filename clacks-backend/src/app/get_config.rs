use crate::app;
use crate::app::{Config, Encoding, Metrics};
use crate::domain::MAX_MESSAGE_LEN_BYTES;
use crate::errors::{Error, Result};
use clacks_macros::application_handler;

#[derive(Clone)]
pub struct GetConfigHandler<E, M> {
    encoding: E,
    metrics: M,
}

impl<E, M> GetConfigHandler<E, M> {
    pub fn new(encoding: E, metrics: M) -> Self {
        Self { encoding, metrics }
    }
}

impl<E, M> app::GetConfigHandler for GetConfigHandler<E, M>
where
    E: Encoding,
    M: Metrics,
{
    #[application_handler]
    fn get_config(&self) -> Result<Config> {
        Ok::<Config, Error>(Config {
            supported_characters: self.encoding.supported_characters(),
            max_message_len_in_bytes: MAX_MESSAGE_LEN_BYTES,
        })
    }
}
