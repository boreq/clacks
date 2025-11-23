use crate::errors::Result;
use anyhow::anyhow;

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    address: String,
    queue_size: usize,
    environment: Environment,
}

impl Config {
    pub fn new(address: impl Into<String>, queue_size: usize, environment: Environment) -> Result<Self> {
        let address = address.into();

        if address.is_empty() {
            return Err(anyhow!("address can't be empty").into());
        }
        if queue_size == 0 {
            return Err(anyhow!("queue size must be positive").into());
        }
        Ok(Self { address, queue_size, environment })
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
}


#[derive(Debug, PartialEq, Eq)]
pub enum Environment {
    Production,
    Development,
}