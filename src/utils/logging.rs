use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn setup_rust_logging() {
    // if std::env::var_os("RUST_LOG").is_none() {
    //     std::env::set_var("RUST_LOG", "NAIA-API=debug,tower_http=debug")
    // }
    // tracing_subscriber::fmt::init();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "naia_api=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
}
