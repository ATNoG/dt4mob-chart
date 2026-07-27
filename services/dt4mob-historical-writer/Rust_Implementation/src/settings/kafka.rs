#[derive(Debug, Clone)]
pub struct KafkaSettings {
    pub bootstrap_servers: String,
    pub security_protocol: String,
    pub ssl_ca_location: String,
    pub ssl_certificate_location: String,
    pub ssl_key_location: String,
    pub consumer_group: String,
    pub topic: String,
    pub auto_offset_reset: String,
    pub session_timeout_ms: String,
    pub heartbeat_interval_ms: String,
}

impl KafkaSettings {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            bootstrap_servers: std::env::var("KAFKA_BOOTSTRAP_SERVERS")?,
            security_protocol: std::env::var("KAFKA_SECURITY_PROTOCOL")?,
            ssl_ca_location: std::env::var("KAFKA_SSL_CA_LOCATION")?,
            ssl_certificate_location: std::env::var("KAFKA_SSL_CERTIFICATE_LOCATION")?,
            ssl_key_location: std::env::var("KAFKA_SSL_KEY_LOCATION")?,
            consumer_group: std::env::var("KAFKA_CONSUMER_GROUP")?,
            topic: std::env::var("KAFKA_TOPIC")?,
            auto_offset_reset: std::env::var("KAFKA_AUTO_OFFSET_RESET")?,
            session_timeout_ms: std::env::var("KAFKA_SESSION_TIMEOUT_MS")
                .unwrap_or_else(|_| "45000".to_string()),
            heartbeat_interval_ms: std::env::var("KAFKA_HEARTBEAT_INTERVAL_MS")
                .unwrap_or_else(|_| "15000".to_string()),
        })
    }

    pub fn as_dict(&self) -> Vec<(&str, &str)> {
        vec![
            ("bootstrap.servers", &self.bootstrap_servers),
            ("security.protocol", &self.security_protocol),
            ("ssl.ca.location", &self.ssl_ca_location),
            ("ssl.certificate.location", &self.ssl_certificate_location),
            ("ssl.key.location", &self.ssl_key_location),
            ("group.id", &self.consumer_group),
            ("auto.offset.reset", &self.auto_offset_reset),
            ("enable.auto.commit", "true"),
            ("session.timeout.ms", &self.session_timeout_ms),
            ("heartbeat.interval.ms", &self.heartbeat_interval_ms),
        ]
    }
}
