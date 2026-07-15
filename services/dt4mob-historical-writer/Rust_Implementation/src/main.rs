mod database_engines;
mod models;
mod services;
mod settings;

use tracing::{info, error};
use tracing_subscriber::EnvFilter;

use database_engines::timescale_engine_manager::TimescaleDBEngineManager;
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

    let db_engine = match TimescaleDBEngineManager::new(&settings.timescale.get_connection()).await
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

    let manager = DittoEventsManager::new(db_engine);

    let consumer = match KafkaConsumer::new(settings.kafka.as_dict(), &settings.kafka.topic) {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to create Kafka consumer: {}", e);
            std::process::exit(1);
        }
    };

    consumer.consume(&manager).await;
}
