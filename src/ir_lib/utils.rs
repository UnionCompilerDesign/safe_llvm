use std::ffi::{CString, NulError};
use std::path::Path;
use std::fs;
use llvm_sys::core;
use llvm_sys::prelude::LLVMModuleRef;

/// Gets the parameter of a function
pub fn get_param(function: *mut llvm::LLVMValue, index: u32) -> *mut llvm::LLVMValue{
    unsafe {
        core::LLVMGetParam(function, index)
    }
}

/// Writes an LLVM module to a file
pub fn write_to_file(module: &LLVMModuleRef, file_name: &str) -> Result<(), String> {
    if module.is_null() {
        return Err("LLVM module reference is null".into());
    }

    let output_dir = Path::new("target");
    let output_file_path = output_dir.join(file_name);

    if !output_dir.exists() {
        fs::create_dir_all(output_dir)
            .map_err(|e| format!("Failed to create target directory: {}", e))?;
    }

    let output_file_cstr = path_to_cstring(&output_file_path)
        .map_err(|e| format!("Failed to convert path to CString: {}", e))?;

    let result = unsafe {
        core::LLVMPrintModuleToFile(module.clone(), output_file_cstr.as_ptr(), std::ptr::null_mut())
    };

    if result == 0 {
        Ok(())
    } else {
        Err("LLVMPrintModuleToFile failed".into())
    }
}

fn path_to_cstring(path: &Path) -> Result<CString, NulError> {
    let path_str = path.to_str()
        .ok_or_else(|| CString::new("").unwrap_err())?; 
    CString::new(path_str) 
}