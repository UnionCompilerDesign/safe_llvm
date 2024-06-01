//! Input/Output utilities for LLVM modules in the IR generator.

extern crate llvm_sys as llvm;
use std::{ffi::{c_char, CStr}, fs, path::Path, sync::{Arc, RwLock}};
use llvm::core;
use crate::{cstring, pointer::{LLVMRef, LLVMRefType, SafeLLVMPointer}};

/// Writes an LLVM module to a file.
///
/// This function serializes an LLVM module, contained inside a `SafeLLVMPointer`, to a specified file.
///
/// # Parameters
/// * `module` - An `Arc<RwLock<SafeLLVMPointer>>` pointing to the LLVM module to be written.
/// * `file_name` - The name of the file where the LLVM IR should be saved.
///
/// # Returns
/// A `Result<(), String>` indicating the success or failure of the operation.
/// Returns `Ok(())` if the module is successfully written to the file.
/// Returns `Err(String)` if there are issues obtaining locks, converting paths, creating directories,
/// or in the LLVM API call to write the module.
pub fn write_ir_to_file(module: Arc<RwLock<SafeLLVMPointer>>, file_name: &str) -> Result<(), String> {
    let module_ref_rwlock = module.read().map_err(|_| "Failed to obtain read lock on module".to_string())?;

    // Extract the LLVMModuleRef from the SafeLLVMPointer
    let module_ptr = module_ref_rwlock.read(LLVMRefType::Module, |llvm_ref| {
        if let LLVMRef::Module(ptr) = llvm_ref {
            Some(*ptr)
        } else {
            None
        }
    }).ok_or_else(|| "Failed to extract LLVM module reference".to_string())?;

    // Define the output directory and file path
    let output_dir = Path::new("target");
    let output_file_path = output_dir.join(file_name);

    // Ensure the output directory exists
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)
            .map_err(|e| format!("Failed to create target directory: {}", e))?;
    }

    // Convert the file path to a CString for LLVM's API
    let output_file_cstr = cstring::convert_path_to_cstring(&output_file_path)
        .map_err(|e| format!("Failed to convert path to CString: {}", e))?;

    // Call LLVM's function to print the module to the specified file
    let result = unsafe {
        core::LLVMPrintModuleToFile(module_ptr, output_file_cstr.as_ptr(), std::ptr::null_mut())
    };

    // Handle the result of the file printing operation
    if result == 0 {
        Ok(())
    } else {
        Err("LLVMPrintModuleToFile failed".into())
    }
}

/// Writes the LLVM module to a string.
///
/// This function serializes an LLVM module, contained inside a `SafeLLVMPointer`, to a string.
///
/// # Parameters
/// * `module` - An `Arc<RwLock<SafeLLVMPointer>>` pointing to the LLVM module to be serialized.
///
/// # Returns
/// A `Result<String, String>` containing the serialized LLVM IR as a string if successful,
/// or an error message if the operation fails.
///
pub fn write_to_string(module: Arc<RwLock<SafeLLVMPointer>>) -> Result<String, String> {
    let module_ref_rwlock = module.read().map_err(|_| "Failed to obtain read lock on module".to_string())?;

    // Extract the LLVMModuleRef from the SafeLLVMPointer
    let module_ptr = module_ref_rwlock.read(LLVMRefType::Module, |llvm_ref| {
        if let LLVMRef::Module(ptr) = llvm_ref {
            Some(*ptr)
        } else {
            None
        }
    }).ok_or_else(|| "Failed to extract LLVM module reference".to_string())?;

    // Print the module to a C style string and convert to &str
    let str_result: Result<&str, std::str::Utf8Error> = unsafe {
        let raw_ptr_str: *mut c_char = core::LLVMPrintModuleToString(module_ptr);
        let c_str_representation: &CStr = CStr::from_ptr(raw_ptr_str);
        c_str_representation.to_str()
    };
    
    // Handle the result of the conversion
    match str_result {
        Ok(string) => Ok(string.to_string()),
        Err(_error) => Err("Writing module to string failed!".into())
    }

}