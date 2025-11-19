pub mod time;

use crate::errors::Error;
use crate::errors::Result;
use anyhow::anyhow;
use std::sync::{Arc, Mutex};

pub enum ShutterPosition {
    Open,
    Closed,
}

pub enum ShutterLocation {
    TopLeft,
    TopRight,
    MiddleLeft,
    MiddleRight,
    BottomLeft,
    BottomRight,
}

pub struct Message {
    text: String,
}

pub struct EncodedMessage {
    parts: Vec<MessagePart>,
}

pub struct MessagePart {
    text: String,
    encoding: ShutterLocation,
}

#[derive(Clone)]
pub struct Queue {
    messages: Arc<Mutex<Vec<Message>>>,
    max_messages: usize,
}

impl Queue {
    pub fn new(max_messages: usize) -> Result<Self> {
        if max_messages == 0 {
            return Err(anyhow!("max_messages in the queue can't be set to zero").into());
        }
        Ok(Self {
            messages: Arc::new(Mutex::new(vec![])),
            max_messages,
        })
    }

    pub fn add_message(&self, message: Message) -> Result<()> {
        let mut messages = self.messages.lock().unwrap();
        if messages.len() >= self.max_messages {
            return Err(Error::QueueIsFull);
        }
        messages.push(message);
        Ok(())
    }

    pub fn pop_message(&self, _message: Message) -> Option<Message> {
        let mut messages = self.messages.lock().unwrap();
        messages.pop()
    }
}
