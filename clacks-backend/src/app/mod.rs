pub mod add_message_to_queue;
pub mod get_state;
pub mod update_clacks;

use crate::domain;
use crate::domain::time::Duration;
use crate::domain::{CurrentMessage, EncodedMessage, Message};
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

pub trait GetStateHandler {
    fn get_state(&self) -> Result<State>;
}

pub struct State {
    current_message: Option<CurrentMessage>,
    queue: Vec<EncodedMessage>,
}

pub trait Clacks {
    fn update(&self) -> Result<ClacksUpdateResult>;
    fn current_message(&self) -> Option<CurrentMessage>;
}

pub enum ClacksUpdateResult {
    StateChanged,
    StateNotChanged,
}

pub trait Queue {
    fn add_message(&self, message: EncodedMessage) -> Result<()>;
    fn pop_message(&self) -> Option<EncodedMessage>;
    fn get_messages(&self) -> Result<Vec<EncodedMessage>>;
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

pub trait EventPublisher {
    fn publish_clacks_updated(&self) -> Result<()>;
    fn publish_message_added_to_queue(&self) -> Result<()>;
}

pub enum ApplicationHandlerCallResult {
    Ok,
    Error,
}

impl Clacks for domain::Clacks {
    fn update(&self) -> Result<ClacksUpdateResult> {
        self.update()
    }

    fn current_message(&self) -> Option<CurrentMessage> {
        self.current_message()
    }
}

impl Queue for domain::Queue {
    fn add_message(&self, message: EncodedMessage) -> Result<()> {
        self.add_message(message)
    }

    fn pop_message(&self) -> Option<EncodedMessage> {
        self.pop_message()
    }

    fn get_messages(&self) -> Result<Vec<EncodedMessage>> {
        self.get_messages()
    }
}

impl Encoding for domain::Encoding {
    fn encode(&self, message: &Message) -> Result<EncodedMessage> {
        self.encode(message)
    }
}
