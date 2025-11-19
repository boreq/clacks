mod add_message_to_queue;

use crate::domain::Message;
use crate::domain::time::Duration;
use crate::errors::Result;

pub struct UpdateClacks {}

impl Default for UpdateClacks {
    fn default() -> Self {
        Self::new()
    }
}

impl UpdateClacks {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait UpdateClacksHandler {
    fn handle(&mut self, update_clacks: UpdateClacks) -> Result<()>;
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
    fn handle(&mut self, add_message_to_queue: AddMessageToQueue) -> Result<()>;
}

pub trait Queue {
    fn add_message(&self, message: Message) -> Result<()>;
    fn pop_message(&self, message: Message) -> Option<Message>;
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
