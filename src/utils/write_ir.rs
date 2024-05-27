extern crate llvm_sys as llvm;

use std::{ffi::CString, fs, path::Path, sync::{Arc, RwLock}};

use crate::{
    memory_management::pointer::{LLVMRef, LLVMRefType, SafeLLVMError, SafeLLVMPointer}, 
    utils::utils_struct::Utils,
};

impl Utils {
    /// Writes an LLVM module to a file
    pub fn write_to_file(module: Arc<RwLock<SafeLLVMPointer>>, file_name: &str) -> Result<(), SafeLLVMError> {
        let module_ref_rwlock = module.read().map_err(|_| SafeLLVMError::LockError("Failed to obtain read lock on module".into()))?;
    
        // Extract the LLVMModuleRef from the SafeLLVMPointer
        let module_ptr_result = module_ref_rwlock.read(LLVMRefType::Module, |llvm_ref| {
            match llvm_ref {
                LLVMRef::Module(ptr) => Some(*ptr),
                _ => None, 
            }
        });

        let module_ptr = match module_ptr_result {
            Ok(Some(ptr)) => ptr,
            _ => return Err(SafeLLVMError::IncorrectPointerType("Failed to extract LLVM module reference".into())),
        };
    
        // Define the output directory and file path
        let output_dir = Path::new("target");
        let output_file_path = output_dir.join(file_name);
    
        // Ensure the output directory exists
        if !output_dir.exists() {
            fs::create_dir_all(output_dir)
                .map_err(|e| SafeLLVMError::InvalidPointer(format!("Failed to create target directory: {}", e)))?;
        }
    
        // Convert the file path to a CString for LLVM's API
        let output_file_cstr = CString::new(output_file_path.to_str().unwrap())
            .map_err(|e| SafeLLVMError::InvalidPointer(format!("Failed to convert path to CString: {}", e)))?;
    
        // Call LLVM's function to print the module to the specified file
        let result = unsafe {
            llvm::core::LLVMPrintModuleToFile(module_ptr, output_file_cstr.as_ptr(), std::ptr::null_mut())
        };
    
        // Handle the result of the file printing operation
        if result == 0 {
            Ok(())
        } else {
            Err(SafeLLVMError::InvalidPointer("LLVMPrintModuleToFile failed".into()))
        }
    }
}