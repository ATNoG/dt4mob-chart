use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Action {
    #[serde(rename = "modified")]
    Modified,
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "merged")]
    Merged,
}

impl TryFrom<&str> for Action {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "modified" | "MODIFIED" => Ok(Action::Modified),
            "created" | "CREATE" => Ok(Action::Created),
            "deleted" | "DELETE" => Ok(Action::Deleted),
            "merged" | "MERGED" => Ok(Action::Merged),
            _ => Err(format!("Unknown action: {}", value)),
        }
    }
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Modified => write!(f, "MODIFIED"),
            Action::Created => write!(f, "CREATE"),
            Action::Deleted => write!(f, "DELETE"),
            Action::Merged => write!(f, "MERGED"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DittoEvent {
    pub time: DateTime<Utc>,
    pub thing_id: String,
    pub action: Action,
    pub revision: Option<i32>,
    pub path: String,
    pub value: Option<Value>,
}

pub const DDL_SQL: &str = r#"
DO $$ 
BEGIN
    CREATE TYPE action_enum AS ENUM ('MODIFIED', 'CREATE', 'DELETE', 'MERGED');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

CREATE TABLE IF NOT EXISTS dittoevent (
    time TIMESTAMPTZ NOT NULL,
    thing_id VARCHAR NOT NULL,
    action action_enum NOT NULL,
    revision INTEGER,
    path VARCHAR NOT NULL,
    value JSONB
);

-- Turn it into a hypertable partitioned only by 'time'
SELECT create_hypertable('dittoevent', 'time', chunk_time_interval => INTERVAL '1 day', if_not_exists => TRUE);

-- Configure TimescaleDB compression
ALTER TABLE dittoevent SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'thing_id',
    timescaledb.compress_orderby = 'time DESC'
);

SELECT add_compression_policy('dittoevent', INTERVAL '7 days', if_not_exists => TRUE);

-- Index for segmentby optimization
CREATE INDEX IF NOT EXISTS idx_dittoevent_thing_id ON dittoevent (thing_id);
"#;
