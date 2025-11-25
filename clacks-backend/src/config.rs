use crate::domain::{Message, TimingConfig};
use crate::errors::Result;
use anyhow::anyhow;

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    address: String,
    queue_size: usize,
    environment: Environment,
    messages_to_inject: Vec<Message>,
    timing: TimingConfig,
}

impl Config {
    pub fn new(
        address: impl Into<String>,
        queue_size: usize,
        environment: Environment,
        messages_to_inject: Vec<Message>,
        timing: TimingConfig,
    ) -> Result<Self> {
        let address = address.into();
        if address.is_empty() {
            return Err(anyhow!("address can't be empty").into());
        }
        if queue_size == 0 {
            return Err(anyhow!("queue size must be positive").into());
        }
        Ok(Self {
            address,
            queue_size,
            environment,
            messages_to_inject,
            timing,
        })
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn queue_size(&self) -> usize {
        self.queue_size
    }

    pub fn environment(&self) -> &Environment {
        &self.environment
    }

    pub fn messages_to_inject(&self) -> &[Message] {
        &self.messages_to_inject
    }

    pub fn timing(&self) -> &TimingConfig {
        &self.timing
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Environment {
    Production,
    Development,
}
