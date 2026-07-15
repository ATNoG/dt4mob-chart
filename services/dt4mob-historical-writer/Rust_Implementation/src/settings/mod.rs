pub mod kafka;
pub mod timescale;

use kafka::KafkaSettings;
use timescale::TimeScale;

#[derive(Debug, Clone)]
pub struct Settings {
    pub log_level: String,
    pub kafka: KafkaSettings,
    pub timescale: TimeScale,
}

impl Settings {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            log_level: std::env::var("LOG_LEVEL").unwrap_or_else(|_| "INFO".to_string()),
            kafka: KafkaSettings::from_env()?,
            timescale: TimeScale::from_env()?,
        })
    }
}
