//! Common functionality and constants for `SafeLLVM`.
//!
//! This crate contains helper modules used across `SafeLLVM` for handling tasks such as
//! I/O operations, string manipulation, pointer management, target configuration, and shared constants.

/// Provides I/O functionalities.
/// This module includes a function to serialize and write LLVM modules to files.
pub mod io;

/// Utilities for handling C strings within Rust.
/// For more details, see the Rust standard library documentation:
/// https://doc.rust-lang.org/std/ffi/struct.CString.html
pub mod cstring;

/// Pointer utilities that manage safe interactions with LLVM pointers.
/// This module provides abstractions to safely wrap and interact with raw LLVM pointers,
/// reducing the risk of unsafe operations.
pub mod pointer;

/// Target configuration utilities.
/// Handles configuration related to LLVM target machines, including setting up and querying
/// target-specific information.
pub mod target;

/// Constants used throughout `SafeLLVM`.
pub mod constants;
