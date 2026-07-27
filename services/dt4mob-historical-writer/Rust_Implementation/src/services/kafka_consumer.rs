use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::Message;
use serde_json::Value;
use tracing::{debug, error, info, warn};

use crate::services::ditto_events_manager::DittoEventsManager;
use crate::services::message_processor::parse_message;

pub struct KafkaConsumer {
    consumer: StreamConsumer,
}

impl KafkaConsumer {
    pub fn new(config: Vec<(&str, &str)>, topic: &str) -> Result<Self, rdkafka::error::KafkaError> {
        let mut client_config = rdkafka::config::ClientConfig::new();
        for (key, value) in config {
            client_config.set(key, value);
        }
        let consumer: StreamConsumer = client_config.create()?;

        consumer.subscribe(&[topic])?;
        info!("Subscribed to {}", topic);

        Ok(Self {
            consumer,
        })
    }

    pub async fn consume(&self, manager: &DittoEventsManager) {
        info!("Staring consumption loop");

        loop {
            match self.consumer.recv().await {
                Err(e) => {
                    error!("Kafka error: {}", e);
                }
                Ok(msg) => {
                    let payload = match msg.payload() {
                        Some(p) => p,
                        None => continue,
                    };

                    let message_text = match std::str::from_utf8(payload) {
                        Ok(t) => t,
                        Err(e) => {
                            error!("UTF-8 decode error: {}", e);
                            continue;
                        }
                    };

                    let message_data: Value = match serde_json::from_str(message_text) {
                        Ok(v) => v,
                        Err(e) => {
                            error!("JSON decode error: {}", e);
                            continue;
                        }
                    };

                    debug!("Received: {}", message_data);

                    let event = match parse_message(&message_data) {
                        Ok(e) => e,
                        Err(e) => {
                            error!("Error parsing message: {}", e);
                            continue;
                        }
                    };

                    debug!("Writing event: time={}, thing_id={}", event.time, event.thing_id);

                    match manager.try_write(event) {
                        Ok(_) => {}
                        Err(tokio::sync::mpsc::error::TrySendError::Full(_)) => {
                            warn!("Channel full, dropping event — batch writer cannot keep up");
                        }
                        Err(tokio::sync::mpsc::error::TrySendError::Closed(_)) => {
                            error!("Channel closed, batch writer is no longer running");
                            break;
                        }
                    }
                }
            }
        }
    }
}
