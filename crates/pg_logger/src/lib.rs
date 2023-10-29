use tracing::info;

pub fn setup_logger() {
    tracing_subscriber::fmt()
        .with_thread_names(true)
        .with_max_level(tracing::Level::INFO)
        .init();
    info!("Tracing subscriber initialized");
}
