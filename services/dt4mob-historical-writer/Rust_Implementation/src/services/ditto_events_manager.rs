use crate::database_engines::timescale_engine_manager::TimescaleDBEngineManager;
use crate::models::ditto_event::DittoEvent;

pub struct DittoEventsManager {
    db_engine: TimescaleDBEngineManager,
}

impl DittoEventsManager {
    pub fn new(db_engine: TimescaleDBEngineManager) -> Self {
        Self { db_engine }
    }

    pub async fn write(&self, event: &DittoEvent) -> Result<(), sqlx::Error> {
        self.db_engine
            .write_event(
                &event.time,
                &event.thing_id,
                &event.action.to_string(),
                event.revision,
                &event.path,
                &event.value,
            )
            .await
    }
}
