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

/// Logging utilities to support internal debugging.
pub mod logging;

pub mod common;

/// Constants 
pub mod constants;

pub mod analysis; 