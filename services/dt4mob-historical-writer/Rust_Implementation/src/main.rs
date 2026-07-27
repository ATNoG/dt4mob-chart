mod database_engines;
mod models;
mod services;
mod settings;

use tracing::{info, error};
use tracing_subscriber::EnvFilter;

use database_engines::timescale_engine_manager::TimescaleDBEngineManager;
use services::batch_writer::run_batch_writer;
use services::ditto_events_manager::DittoEventsManager;
use services::kafka_consumer::KafkaConsumer;
use settings::Settings;

#[tokio::main]
async fn main() {
    std::panic::set_hook(Box::new(|info| {
        eprintln!("PANIC: {}", info);
    }));

    dotenvy::dotenv().ok();

    let settings = match Settings::from_env() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to load settings: {}", e);
            std::process::exit(1);
        }
    };

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(&settings.log_level)),
        )
        .init();

    info!("Starting dt4mob-historical-writer (Rust)");

    let db_engine = match TimescaleDBEngineManager::new(
        &settings.timescale.get_connection(),
        settings.timescale.max_connections,
        settings.timescale.acquire_timeout_ms,
    )
    .await
    {
        Ok(engine) => engine,
        Err(e) => {
            error!("Failed to connect to TimescaleDB: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = db_engine.init_db().await {
        error!("Failed to initialize database: {}", e);
        std::process::exit(1);
    }

    let (tx, rx) = tokio::sync::mpsc::channel(settings.batch.channel_capacity);
    let manager = DittoEventsManager::new(tx);

    let batch_engine = db_engine.pool.clone();
    let batch_size = settings.batch.batch_size;
    let flush_interval_ms = settings.batch.flush_interval_ms;

    tokio::spawn(async move {
        run_batch_writer(&TimescaleDBEngineManager { pool: batch_engine }, rx, batch_size, flush_interval_ms).await;
    });

    let consumer = match KafkaConsumer::new(settings.kafka.as_dict(), &settings.kafka.topic) {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to create Kafka consumer: {}", e);
            std::process::exit(1);
        }
    };

    consumer.consume(&manager).await;
}
