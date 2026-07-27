#[derive(Debug, Clone)]
pub struct TimeScale {
    #[allow(dead_code)]
    pub database_timezone: String,
    pub connection: String,
    pub max_connections: u32,
    pub acquire_timeout_ms: u64,
}

impl TimeScale {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            database_timezone: std::env::var("TIMESCALE_DATABASE_TIMEZONE")
                .unwrap_or_else(|_| "Europe/Lisbon".to_string()),
            connection: std::env::var("TIMESCALE_CONNECTION")?,
            max_connections: std::env::var("DB_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
            acquire_timeout_ms: std::env::var("DB_ACQUIRE_TIMEOUT_MS")
                .unwrap_or_else(|_| "5000".to_string())
                .parse()
                .unwrap_or(5000),
        })
    }

    pub fn get_connection(&self) -> String {
        self.connection
            .replace("postgresql://", "postgresql://")
    }
}
