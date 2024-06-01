//! Utility functions for working with C-style strings in Rust, interfacing with LLVM's API.

extern crate llvm_sys as llvm;
use std::{ffi::{CString, NulError}, path::Path};

/// Converts a Rust `Path` object to a C-style string (`CString`).
///
/// # Parameters
/// * `path` - A reference to the `Path` that should be converted.
///
/// # Returns
/// A `Result` containing a `CString` if conversion is successful, or a `NulError` if the path
/// contains a null byte, which cannot be converted into a `CString`.
///
/// # Errors
/// Returns `NulError` if the path contains an interior null byte, as this invalidates the creation
/// of a `CString`.
pub fn convert_path_to_cstring(path: &Path) -> Result<CString, NulError> {
    let path_str = path.to_str()
        .ok_or_else(|| CString::new("").unwrap_err())?; // Generate a NulError if path is not valid UTF-8
    CString::new(path_str) 
}
