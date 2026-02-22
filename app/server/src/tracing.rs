use crate::config::{Config, LogFormat};
use tracing_subscriber::{
    EnvFilter, Layer, fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt,
};

pub fn init(config: &Config) {
    let fmt_layer: Box<dyn Layer<_> + Send + Sync> = match config.log_format {
        LogFormat::Text => tracing_subscriber::fmt::layer()
            .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
            .boxed(),
        LogFormat::Json => tracing_subscriber::fmt::layer()
            .json()
            .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
            .boxed(),
    };

    tracing_subscriber::registry()
        .with(EnvFilter::new(config.log_level.to_string()))
        .with(fmt_layer)
        .init();
}
