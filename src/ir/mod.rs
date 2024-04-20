/// Initialize IR generation
pub mod builder;

/// Create LLVM module element
pub mod values;

/// Create operations
pub mod ops;

/// Create a type
pub mod types;

/// Create a variable
pub mod var;

/// Maintain block constructions
pub mod block;

/// TLE related utilities
pub mod top_level_exp;

pub mod init;

pub mod api;

pub use api::SafeLLVM;