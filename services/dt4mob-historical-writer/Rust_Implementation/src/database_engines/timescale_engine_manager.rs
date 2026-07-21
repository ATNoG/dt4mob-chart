use sqlx::postgres::PgPool;
use tracing::{debug, info};

use crate::models::ditto_event::DDL_SQL;

pub struct TimescaleDBEngineManager {
    pub pool: PgPool,
}

impl TimescaleDBEngineManager {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn init_db(&self) -> Result<(), sqlx::Error> {
        info!("Initializing database tables and TimescaleDB features");

        // raw_sql executes multi-statement SQL files and preserves DO $$ blocks without breaking on ';'
        sqlx::raw_sql(DDL_SQL).execute(&self.pool).await?;

        info!("Database initialization complete");
        Ok(())
    }

    pub async fn write_event(
        &self,
        time: &chrono::DateTime<chrono::Utc>,
        thing_id: &str,
        action: &str,
        revision: Option<i32>,
        path: &str,
        value: &Option<serde_json::Value>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO dittoevent (time, thing_id, action, revision, path, value)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(time)
        .bind(thing_id)
        .bind(action)
        .bind(revision)
        .bind(path)
        .bind(value)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
