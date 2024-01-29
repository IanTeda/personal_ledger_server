//! ./src/telemetry.rs

use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

/// Register a subscriber as global default to process span data.
/// It should only be called once!
pub fn init(subscriber: impl Subscriber + Sync + Send) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}

/// Compose multiple layers into a `tracing`'s subscriber, env_filter,
/// JsonStorageLayer and formatting layer
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Sync + Send
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{


    // pub const FOO_QTY: usize = if cfg!(build = "release") { 1000 } else { 2 };

    // tracing::info!("FOO_QTY is {}", FOO_QTY);

    // #[cfg(not(build = "release"))]
    // tracing::info!("Not relase build");

    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    #[cfg(not(build = "release"))]
    let subscriber = Registry::default()
        .with(env_filter);

    #[cfg(build = "release")]
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    subscriber
}
