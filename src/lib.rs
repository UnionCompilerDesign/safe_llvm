#![warn(clippy::all)]  
/*  
    Enable Clippy with all warnings for quality control. 
    https://doc.rust-lang.org/stable/clippy/usage.html
*/

extern crate llvm_sys as llvm;

/// Provides abstractions for building LLVM Intermediate Representation (IR).
pub mod ir;

/// Offers tools for Just-In-Time (JIT) compilation.
pub mod jit;

/// Manages handling LLVM objects safely.
pub mod memory_management;

/// Logging utilities to support internal debugging.
pub mod logger;

/// Helper utilities for common operations throughout the library.
pub mod utils;

/// Interface for external modules or libraries to interact with this project's functionalities.
pub mod interface;

// pub mod lld;  // TODO support for the LLVM LLD linker.

// pub mod serialization;  // TODO serialization support of LLVM data structures.

// pub mod debug;  // TODO debugging utilities for internal and external diagnostics.

// pub mod aot;  // TODO tools related to Ahead-Of-Time compilation capabilities.

// pub mod analysis;  // TODO analysis tools based on LLVM's analysis capabilities.

/// High-level API module to allow for mock testing.
pub mod api;

// pub use api::SafeLLVM;  // Exports the public api.
