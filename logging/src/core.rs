use std::fs::File;

use slog::{error, info, o, warn, Drain, Duplicate, Fuse, Logger};
use slog_async::Async;
use slog_json::Json;
use slog_term::{CompactFormat, TermDecorator};

pub fn log_info(logger: &Logger, msg: &str) {
    info!(logger, "{}", msg);
}

pub fn log_warning(logger: &Logger, msg: &str) {
    warn!(logger, "{}", msg);
}

pub fn log_error(logger: &Logger, msg: &str) {
    error!(logger, "{}", msg);
}

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