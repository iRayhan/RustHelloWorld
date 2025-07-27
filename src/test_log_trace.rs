use tracing::{debug, error, info, warn};

pub fn subscribe_log() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

pub fn print_info(data: &str) {
    info!(data);
}

pub fn print_warn(data: &str) {
    warn!(data);
}

pub fn print_debug(data: &str) {
    debug!(data);
}

pub fn print_error(data: &str) {
    error!(data);
}
