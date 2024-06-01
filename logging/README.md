# Logging Utilities for SafeLLVM

## Overview
The `logging` module in the SafeLLVM project provides logging functionalities using the `slog` library. This module is designed to support flexible logging across for terminal and file outputs.

## Features
- **Asynchronous Logging:** Leverages asynchronous logging.
- **Multiple Output Channels:** Configurs terminal and JSON outputs.

## Usage
Here's how to utilize the logging functionalities provided in the `logging` module:

### Initialize the Logger
Before logging any messages, initialize the logger:
```rust
    use safe_llvm::logging;

    let logger = logging::core::init_logger();
    println!("Logger has been initialized successfully.");
```

### Logging Messages
After initializing the logger, you can log messages at different severity levels:
```rust
    // Logging an info message
    logging::core::log_info(&logger, "This is an info message.");

    // Logging a warning message
    logging::core::log_warning(&logger, "This is a warning message.");

    // Logging an error message
    logging::core::log_error(&logger, "This is an error message.");
```

## FAQ

## Further Information
For further information or questions regarding the use of the logging module in SafeLLVM, feel free to contact the main contributors or raise an issue on the GitHub repository.