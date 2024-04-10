extern crate llvm_sys as llvm;

use std::{ffi::{CString, NulError}, path::Path};

pub fn convert_path_to_cstring(path: &Path) -> Result<CString, NulError> {
    let path_str = path.to_str()
        .ok_or_else(|| CString::new("").unwrap_err())?; 
    CString::new(path_str) 
}