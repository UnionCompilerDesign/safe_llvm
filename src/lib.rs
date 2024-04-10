extern crate llvm_sys as llvm;

/// LLVM IR code generation
pub mod ir_codegen;

/// Just in time compilation tools
pub mod jit;

/// Memory management
pub mod memory_management;

/// LLD linker
pub mod lld;

/// LLVM Optimization passes
pub mod optimization;

pub mod serialization;

/// Initialialize LLVM toolchain
pub mod init;

/// Debug utilities
pub mod debug;

/// Clang interface
pub mod clang;

pub mod bitcode;

/// Ahead of time compilation tools
pub mod aot;

/// LLVM analysis
pub mod analysis;

pub mod utils;

pub mod api;