use std::time::Duration;

use tokio::sync::mpsc;
use tokio::time::interval;
use tracing::{debug, error, info};

use crate::database_engines::timescale_engine_manager::TimescaleDBEngineManager;
use crate::models::ditto_event::DittoEvent;

pub async fn run_batch_writer(
    db_engine: &TimescaleDBEngineManager,
    mut rx: mpsc::Receiver<DittoEvent>,
    batch_size: usize,
    flush_interval_ms: u64,
) {
    let mut buffer: Vec<DittoEvent> = Vec::with_capacity(batch_size);
    let mut tick = interval(Duration::from_millis(flush_interval_ms));

    info!(
        "Batch writer started (batch_size={}, flush_interval={}ms)",
        batch_size, flush_interval_ms
    );

    loop {
        tokio::select! {
            event = rx.recv() => {
                match event {
                    Some(event) => {
                        buffer.push(event);
                        if buffer.len() >= batch_size {
                            flush(db_engine, &mut buffer).await;
                        }
                    }
                    None => {
                        info!("Channel closed, flushing remaining {} events", buffer.len());
                        flush(db_engine, &mut buffer).await;
                        break;
                    }
                }
            }
            _ = tick.tick() => {
                if !buffer.is_empty() {
                    flush(db_engine, &mut buffer).await;
                }
            }
        }
    }

    info!("Batch writer stopped");
}

async fn flush(db_engine: &TimescaleDBEngineManager, buffer: &mut Vec<DittoEvent>) {
    let events: Vec<DittoEvent> = buffer.drain(..).collect();
    let count = events.len();
    debug!("Flushing batch of {} events", count);

    if let Err(e) = db_engine.write_events(&events).await {
        error!("Error writing batch of {} events to database: {}", count, e);
    }
}
