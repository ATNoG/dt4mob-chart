#[derive(Debug, Clone)]
pub struct TimeScale {
    #[allow(dead_code)]
    pub database_timezone: String,
    pub connection: String,
}

impl TimeScale {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            database_timezone: std::env::var("TIMESCALE_DATABASE_TIMEZONE")
                .unwrap_or_else(|_| "Europe/Lisbon".to_string()),
            connection: std::env::var("TIMESCALE_CONNECTION")?,
        })
    }

    pub fn get_connection(&self) -> String {
        self.connection
            .replace("postgresql://", "postgresql://")
    }
}
