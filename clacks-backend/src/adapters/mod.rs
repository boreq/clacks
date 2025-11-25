use crate::app;
use crate::app::ApplicationHandlerCallResult;
use crate::config::{Config, Environment};
use crate::domain::TimingConfig;
use crate::domain::time::Duration;
use crate::errors::{Error, Result};
use anyhow::anyhow;
use prometheus::{CounterVec, HistogramOpts, HistogramVec, Opts, Registry, labels};
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Receiver;

pub struct ConfigLoader {
    path: PathBuf,
}

impl ConfigLoader {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self { path: path.into() }
    }

    pub fn load(&self) -> Result<Config> {
        let content = fs::read_to_string(&self.path)?;
        let transport: TomlConfig = toml::from_str(&content)?;
        Config::try_from(transport)
    }
}

#[derive(Deserialize)]
struct TomlConfig {
    address: String,
    queue_size: usize,
    environment: String,
    timing: TomlTimingConfig,
}

impl TryFrom<TomlConfig> for Config {
    type Error = crate::errors::Error;

    fn try_from(value: TomlConfig) -> std::result::Result<Self, Self::Error> {
        Config::new(
            value.address,
            value.queue_size,
            value.environment.try_into()?,
            value.timing.try_into()?,
        )
    }
}

impl TryFrom<String> for Environment {
    type Error = crate::errors::Error;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        match value.as_str() {
            "production" => Ok(Environment::Production),
            "development" => Ok(Environment::Development),
            other => Err(anyhow!("invalid environment: {}", other).into()),
        }
    }
}

#[derive(Deserialize)]
struct TomlTimingConfig {
    show_character_for: u64,
    pause_between_characters_for: u64,
    pause_between_messages_for: u64,
}

impl TryFrom<TomlTimingConfig> for TimingConfig {
    type Error = crate::errors::Error;

    fn try_from(value: TomlTimingConfig) -> std::result::Result<Self, Self::Error> {
        Ok(TimingConfig::new(
            Duration::new_from_seconds(value.show_character_for),
            Duration::new_from_seconds(value.pause_between_characters_for),
            Duration::new_from_seconds(value.pause_between_messages_for),
        ))
    }
}

#[derive(Clone)]
pub struct Metrics {
    registry: Registry,

    metric_application_handler_calls_counter: CounterVec,
    metric_application_handler_calls_histogram: HistogramVec,
}

impl Metrics {
    pub fn new() -> Result<Self> {
        let registry = Registry::new_custom(Some("bricked".into()), None)?;

        let metric_application_handler_calls_counter = CounterVec::new(
            Opts::new(
                "application_handler_calls_counter",
                "application handler calls counter",
            ),
            &["handler_name", "result"],
        )?;
        registry.register(Box::new(metric_application_handler_calls_counter.clone()))?;

        let metric_application_handler_calls_histogram = HistogramVec::new(
            HistogramOpts::new(
                "application_handler_calls_histogram",
                "application handler calls durations",
            ),
            &["handler_name", "result"],
        )?;
        registry.register(Box::new(metric_application_handler_calls_histogram.clone()))?;

        Ok(Self {
            registry,

            metric_application_handler_calls_counter,
            metric_application_handler_calls_histogram,
        })
    }

    pub fn registry(&self) -> &Registry {
        &self.registry
    }
}

impl app::Metrics for Metrics {
    fn record_application_handler_call(
        &self,
        handler_name: &str,
        result: ApplicationHandlerCallResult,
        duration: Duration,
    ) {
        let labels = labels! {
            "handler_name" => handler_name,
            "result" => match result {
                ApplicationHandlerCallResult::Ok => "ok",
                ApplicationHandlerCallResult::Error => "error"
            },
        };

        self.metric_application_handler_calls_counter
            .with(&labels)
            .inc();

        self.metric_application_handler_calls_histogram
            .with(&labels)
            .observe(duration.as_seconds());
    }
}

#[derive(Clone)]
pub struct PubSub {
    clacks_updated: broadcast::Sender<()>,
    _clacks_updated_receiver: Arc<broadcast::Receiver<()>>,
    message_added_to_queue: broadcast::Sender<()>,
    _message_added_to_queue_receiver: Arc<broadcast::Receiver<()>>,
}

impl Default for PubSub {
    fn default() -> Self {
        Self::new()
    }
}

impl PubSub {
    pub fn new() -> Self {
        let (clacks_updated, clacks_updated_receiver) = broadcast::channel(1);
        let (message_added_to_queue, message_added_to_queue_receiver) = broadcast::channel(1);

        Self {
            clacks_updated,
            _clacks_updated_receiver: Arc::new(clacks_updated_receiver),
            message_added_to_queue,
            _message_added_to_queue_receiver: Arc::new(message_added_to_queue_receiver),
        }
    }
}

impl app::EventPublisher for PubSub {
    fn publish_clacks_updated(&self) -> Result<()> {
        self.clacks_updated
            .send(())
            .map_err(|e| Error::Unknown(anyhow!(e)))?;
        Ok(())
    }

    fn publish_message_added_to_queue(&self) -> Result<()> {
        self.message_added_to_queue
            .send(())
            .map_err(|e| Error::Unknown(anyhow!(e)))?;
        Ok(())
    }
}

impl PubSub {
    pub fn subscribe_to_clacks_updated(&self) -> Receiver<()> {
        self.clacks_updated.subscribe()
    }

    pub fn subscribe_to_message_added_to_queue(&self) -> Receiver<()> {
        self.message_added_to_queue.subscribe()
    }
}

#[cfg(test)]
mod tests {
    use super::ConfigLoader;
    use super::*;
    use crate::config::Config;
    use crate::fixtures;

    #[test]
    fn loads_config_from_file_successfully() -> Result<()> {
        let expected_config = Config::new(
            "0.0.0.0:8080",
            10,
            Environment::Development,
            TimingConfig::new(
                Duration::new_from_seconds(1),
                Duration::new_from_seconds(2),
                Duration::new_from_seconds(3),
            ),
        )?;
        let loader = ConfigLoader::new(fixtures::test_file_path(
            "src/adapters/testdata/config.toml",
        ));
        let config = loader.load()?;
        assert_eq!(expected_config, config);
        Ok(())
    }
}
