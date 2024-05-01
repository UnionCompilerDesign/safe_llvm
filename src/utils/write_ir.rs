extern crate llvm_sys as llvm;

use std::{fs, path::Path};

use llvm::{core, prelude::LLVMModuleRef};

use crate::utils::{cstring, utils_struct::Utils};

impl Utils {
    /// Writes an LLVM module to a file
    pub fn write_to_file(module: &LLVMModuleRef, file_name: &str) -> Result<(), String> {
        let output_dir = Path::new("target");
        let output_file_path = output_dir.join(file_name);

        if !output_dir.exists() {
            fs::create_dir_all(output_dir)
                .map_err(|e| format!("Failed to create target directory: {}", e))?;
        }

        let output_file_cstr = cstring::convert_path_to_cstring(&output_file_path)
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
}