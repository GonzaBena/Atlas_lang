use log::{debug, error, info, warn};

pub fn init_logging() {
    // Initializes the logger with the default log level. configure via RUST_LOG env variable
    env_logger::init();
}
