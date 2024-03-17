//! Setup the API log telemetry
//!
//! # APPLICATION TELEMETRY
//!
//! Instrumenting to collect structured, event-based diagnostic information.
//!
//! Tracing is made up of spans, events within those spans and subscribers that
//! pick what spans and events to grab and and what then performs tasks on the
//! grabbed spans and events.
//!
//! ## REFERENCES
//!
//! Learn more about Rust Telemetry (i.e async logging)
//!
//! * [Tracing Crate Documentation](https://docs.rs/tracing/latest/tracing/)
//! * [Tracing Repo](https://github.com/tokio-rs/tracing)
//! * [Decrusting the tracing crate](https://youtu.be/21rtHinFA40)
//! * [Getting started with Tracing](https://tokio.rs/tokio/topics/tracing)
//! * [Can we have easier pretty log for development?](https://github.com/LukeMathWalker/tracing-bunyan-formatter/issues/17)

// TODO: Add https://prometheus.io/
// TODO: Add https://opentelemetry.io/
// TODO: Add tracing console

use crate::configuration;
use tracing::subscriber::set_global_default;
use tracing::{debug, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Layer, Registry};

/// Compose multiple subscriber layers into a `tracing` subscriber registry.
/// 
/// # GET TRACING SUBSCRIBER
///
/// Compose multiple subscriber layers into a `tracing` subscriber registry.
///
/// We are using an `impl Subscriber` return type to avoid having to spell out
/// the actual type of the returned subscriber, which is indeed quite complex.
///
/// ## ARGUMENTS
///
/// * `name` - Name to append to all span and event formatted records.
/// * `sink` - Write formatted records to the sink.
/// * `env` - Takes a personal_ledger_server::configuration::Env enum value
/// * `log_level` - Takes a Takes a personal_ledger_server::configuration::LogLevels enum value
///
pub fn get_tracing_subscriber<Sink>(
    name: String,
    sink: Sink,
    env: configuration::Env,
    log_level: configuration::LogLevels,
) -> impl Subscriber + Sync + Send
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    // We are falling back to printing all spans at configuration log level or
    // above if the RUST_LOG environment variable has not been set.
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(log_level));

    // When running in a development environment, output records to pretty std.out
    let emit_pretty = env == configuration::Env::Development;
    let pretty_formatting_layer = tracing_subscriber::fmt::layer()
        // .pretty()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE) // Capture Actix span events
        .with_filter(filter_fn(move |_| emit_pretty));

    // When running in a Production environment, output records in JSON
    let emit_bunyan = env == configuration::Env::Production;
    let bunyan_json_layer = JsonStorageLayer
        .with_filter(filter_fn(move |_| emit_bunyan));
    let bunyan_formatting_layer =
        BunyanFormattingLayer::new(name, sink).with_filter(filter_fn(move |_| emit_bunyan));

    // TODO: Add console subscriber
    // let console_subscriber =
    //     console_subscriber::TasksLayer::builder().build();

    // A subscriber registry of tracing layers.
    Registry::default()
        .with(env_filter)
        .with(pretty_formatting_layer)
        .with(bunyan_json_layer)
        .with(bunyan_formatting_layer)
}

/// Register the tracing subscriber(s) to capture and process events and spans.
/// 
/// # INITIATE TRACING
///
/// Register the tracing subscriber(s) to capture and process events and spans.
///  
/// It should only be called once!
///
/// ## ARGUMENTS
///
/// * `subscribers` - A registry of tracing subscribers.
///
pub fn init_tracing(subscribers: impl Subscriber + Sync + Send, log_level: configuration::LogLevels) {
    // Convert all log records into tracing events.
    LogTracer::init().expect("Failed to set logger");

    // Set subscriber that should be used to process events and spans.
    set_global_default(subscribers).expect("Failed to set subscriber");

    debug!(
        "Log tracing initiated at {} level and above.",
        log_level
    );
}
