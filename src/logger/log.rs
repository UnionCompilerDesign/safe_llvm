use slog::{error, info, warn, Logger};

pub fn log_info(logger: &Logger, msg: &str) {
    info!(logger, "{}", msg);
}

pub fn log_warning(logger: &Logger, msg: &str) {
    warn!(logger, "{}", msg);
}

pub fn log_error(logger: &Logger, msg: &str) {
    error!(logger, "{}", msg);
}