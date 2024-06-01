//! `SafeLLVM` aims to be a safe wrapper for the LLVM toolchain.
//!
//! This crate is designed to tie together various functionalities like IR generation, JIT compilation,
//! logging, etc, for managing LLVM processes within a single package.
//!
//! ## Modules
//!
//! - `analysis`: Provides functionalities for analyzing and processing LLVM IR.
//! - `common`: Contains common utilities and helpers used across the project.
//! - `ir`: Responsible for the generation and manipulation of LLVM Intermediate Representation (IR).
//! - `jit`: Manages Just-In-Time (JIT) compilation features leveraging LLVM's JIT compilers.
//! - `logging`: Facilitates logging across all modules, supporting both terminal and file-based outputs.

pub use analysis;
pub use common;
pub use ir;
pub use jit;
pub use logging;
