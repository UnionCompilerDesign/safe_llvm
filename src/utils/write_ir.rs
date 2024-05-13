extern crate llvm_sys as llvm;

use std::{fs, path::Path, sync::{Arc, RwLock}};

use llvm::core;

use crate::{
    memory_management::pointer::{SafeLLVMPointer, LLVMRef, LLVMRefType}, 
    utils::{cstring, utils_struct::Utils}
};

impl Utils {
    /// Writes an LLVM module to a file
    pub fn write_to_file(module: Arc<RwLock<SafeLLVMPointer>>, file_name: &str) -> Result<(), String> {
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
}