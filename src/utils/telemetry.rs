use tracing_subscriber::{fmt, prelude::*};
use tracing_subscriber::filter::EnvFilter;

pub fn init_xiro_telemetry() {
    tracing_subscriber::registry()
        .with(fmt::layer()
            .with_target(false)
            .with_thread_ids(false)
            .with_level(true)
            .compact())
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();
}