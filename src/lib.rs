#![warn(clippy::all)]

extern crate llvm_sys as llvm;

/// LLVM IR code generation
pub mod ir;

/// Just in time compilation tools
pub mod jit;

/// Memory management
pub mod memory_management;

/// LLD linker
pub mod lld;

pub mod serialization;

/// Debug utilities
pub mod debug;

/// Ahead of time compilation tools
pub mod aot;

/// LLVM analysis
pub mod analysis;

pub mod utils;

pub mod interface;

pub mod api;

// pub use api::SafeLLVM;