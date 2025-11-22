pub mod add_message_to_queue;
pub mod update_clacks;

use crate::domain::{EncodedMessage, Message};
use crate::domain::time::Duration;
use crate::errors::Result;

pub trait UpdateClacksHandler {
    fn handle(&self) -> Result<()>;
}

pub struct AddMessageToQueue {
    message: Message,
}

impl AddMessageToQueue {
    pub fn new(message: Message) -> Self {
        Self { message }
    }
}

pub trait AddMessageToQueueHandler {
    fn handle(&self, add_message_to_queue: AddMessageToQueue) -> Result<()>;
}

pub trait Clacks {
    fn update(&self) -> Result<()>;
}

pub trait Queue {
    fn add_message(&self, message: EncodedMessage) -> Result<()>;
    fn pop_message(&self, message: EncodedMessage) -> Option<EncodedMessage>;
}


pub trait Encoding {
    fn encode(&self, message: &Message) -> Result<EncodedMessage>;
}

pub trait Metrics {
    fn record_application_handler_call(
        &self,
        handler_name: &str,
        result: ApplicationHandlerCallResult,
        duration: Duration,
    );
}

pub enum ApplicationHandlerCallResult {
    Ok,
    Error,
}
