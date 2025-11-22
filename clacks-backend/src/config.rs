use crate::errors::Result;
use anyhow::anyhow;

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    queue_size: usize,
}

impl Config {
    pub fn new(queue_size: usize) -> Result<Self> {
        if queue_size == 0 {
            return Err(anyhow!("queue size must be positive").into());
        }
        Ok(Self { queue_size })
    }

    pub fn queue_size(&self) -> usize {
        self.queue_size
    }
}
