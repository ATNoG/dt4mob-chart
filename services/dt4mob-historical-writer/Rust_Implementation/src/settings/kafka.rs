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
        ]
    }
}
