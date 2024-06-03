//! Provides logging functionalities for the application.
//!
//! This module defines several utility functions for logging various levels of messages
//! and a function to initialize a global logger. It uses `slog` for structured, asynchronous logging.

use std::fs::File;
use slog::{error, info, o, warn, Drain, Duplicate, Fuse, Logger};
use slog_async::Async;
use slog_json::Json;
use slog_term::{CompactFormat, TermDecorator};

/// Logs informational messages using the provided logger.
///
/// # Arguments
/// * `logger` - Reference to the logger.
/// * `msg` - Message to log.
pub fn log_info(logger: &Logger, msg: &str) {
    info!(logger, "{}", msg);
}

/// Logs warnings using the provided logger.
///
/// # Arguments
/// * `logger` - Reference to the logger.
/// * `msg` - Warning message to log.
pub fn log_warning(logger: &Logger, msg: &str) {
    warn!(logger, "{}", msg);
}

/// Logs error messages using the provided logger.
///
/// # Arguments
/// * `logger` - Reference to the logger.
/// * `msg` - Error message to log.
pub fn log_error(logger: &Logger, msg: &str) {
    error!(logger, "{}", msg);
}

/// Initializes and returns a logger with both terminal and file (JSON format) outputs.
///
/// This logger is asynchronous and writes logs to `app.log` in the JSON format and also outputs to the terminal in a compact format.
///
/// # Returns
/// A `Logger` instance ready for use throughout the application.
pub fn init_logger() -> Logger {
    // Terminal output drain
    let decorator: TermDecorator = TermDecorator::new().build();
    let term_drain: Fuse<CompactFormat<TermDecorator>> = CompactFormat::new(decorator).build().fuse();

    // File output for JSON 
    let file: File = File::create("app.log").unwrap();
    let json_drain = Json::new(file)
        .add_default_keys() 
        .build()
        .fuse();

    // Make the logging asynchronous
    let drain: Fuse<Duplicate<Fuse<CompactFormat<TermDecorator>>, Fuse<Json<File>>>> = Duplicate::new(term_drain, json_drain).fuse();
    let async_drain: Fuse<Async> = Async::new(drain).build().fuse();

    Logger::root(async_drain, o!("version" => env!("CARGO_PKG_VERSION")))
}
