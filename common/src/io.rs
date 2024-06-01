//! Input/Output utilities for LLVM modules in the IR generator.

extern crate llvm_sys as llvm;
use std::{fs, path::Path, sync::{Arc, RwLock}};
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
///
/// # Errors
/// This function returns an error if:
/// - It fails to acquire a read lock on the LLVM module.
/// - It cannot extract the LLVM module reference from the `SafeLLVMPointer`.
/// - The output directory does not exist and cannot be created.
/// - The file path conversion to `CString` fails.
/// - The LLVM API function `LLVMPrintModuleToFile` returns a non-zero value indicating failure.
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
