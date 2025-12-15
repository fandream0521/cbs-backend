use tower_http::trace::TraceLayer;
use tracing_subscriber::{EnvFilter, Registry, fmt, layer::SubscriberExt};

pub fn init_tracing() -> anyhow::Result<()> {
    if tracing::subscriber::set_global_default(build_subscriber()).is_err() {
        // Already set (e.g., in tests); ignore.
    }
    Ok(())
}

fn build_subscriber() -> impl tracing::Subscriber {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,tower_http=info,sqlx=info"));
    Registry::default().with(env_filter).with(fmt::layer())
}

pub fn trace_layer()
-> TraceLayer<tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>>
{
    TraceLayer::new_for_http()
}
