#[derive(Debug, Clone)]
pub struct BatchSettings {
    pub batch_size: usize,
    pub flush_interval_ms: u64,
    pub channel_capacity: usize,
}

impl BatchSettings {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            batch_size: std::env::var("BATCH_SIZE")
                .unwrap_or_else(|_| "1000".to_string())
                .parse()
                .unwrap_or(1000),
            flush_interval_ms: std::env::var("FLUSH_INTERVAL_MS")
                .unwrap_or_else(|_| "2000".to_string())
                .parse()
                .unwrap_or(2000),
            channel_capacity: std::env::var("CHANNEL_CAPACITY")
                .unwrap_or_else(|_| "10000".to_string())
                .parse()
                .unwrap_or(10000),
        })
    }
}
