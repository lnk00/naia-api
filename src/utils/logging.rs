pub fn setup_rust_logging() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "NAIA-API=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();
}
