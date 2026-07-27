use std::time::Duration;

use sqlx::postgres::{PgPool, PgPoolOptions};
use tracing::{debug, info};

use crate::models::ditto_event::{DDL_SQL, DittoEvent};

pub struct TimescaleDBEngineManager {
    pub pool: PgPool,
}

impl TimescaleDBEngineManager {
    pub async fn new(
        database_url: &str,
        max_connections: u32,
        acquire_timeout_ms: u64,
    ) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .acquire_timeout(Duration::from_millis(acquire_timeout_ms))
            .idle_timeout(Duration::from_secs(300))
            .max_lifetime(Duration::from_secs(1800))
            .connect(database_url)
            .await?;
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
            VALUES ($1, $2, $3::action_enum, $4, $5, $6)
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

    pub async fn write_events(&self, events: &[DittoEvent]) -> Result<(), sqlx::Error> {
        if events.is_empty() {
            return Ok(());
        }

        let sql = Self::build_batch_query(events.len());
        debug!("Batch INSERT with {} events, SQL length: {}", events.len(), sql.len());

        let mut query = sqlx::query(&sql);

        for event in events {
            query = query
                .bind(event.time)
                .bind(&event.thing_id)
                .bind(event.action.to_string())
                .bind(event.revision)
                .bind(&event.path)
                .bind(&event.value);
        }

        query.execute(&self.pool).await?;
        Ok(())
    }

    fn build_batch_query(row_count: usize) -> String {
        let mut sql = String::from(
            "INSERT INTO dittoevent (time, thing_id, action, revision, path, value) VALUES ",
        );

        for i in 0..row_count {
            if i > 0 {
                sql.push_str(", ");
            }
            let base = i * 6;
            sql.push_str(&format!(
                "(${}, ${}, ${}::action_enum, ${}, ${}, ${})",
                base + 1,
                base + 2,
                base + 3,
                base + 4,
                base + 5,
                base + 6
            ));
        }

        sql
    }
}
