use slog::{o, Drain, Duplicate, Fuse, Logger};
use slog_async::Async;
use slog_term::{TermDecorator, CompactFormat};
use slog_json::Json;

use std::fs::File;

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

pub struct ShutdownLogger;

impl Drop for ShutdownLogger {
    fn drop(&mut self) {
        slog_scope::info!("Shutting down development toolchain.");
    }
}
