extern crate llvm_sys as llvm;

use std::ffi::CString;

use llvm::{
        core::{
            LLVMBuildAlloca, LLVMBuildStore, LLVMBuildLoad2,
        }, 
        prelude::{
            LLVMBuilderRef, LLVMContextRef, LLVMTypeRef, LLVMValueRef,
        }
};

/// Initializes a variable
pub fn init_var(
    builder: LLVMBuilderRef,
    _context: LLVMContextRef,
    var_name: &str,
    data_type: LLVMTypeRef,
    initial_value: Option<LLVMValueRef>, 
) -> LLVMValueRef {
    let var_name_cstr = CString::new(var_name).unwrap();
    let alloca = unsafe {
        LLVMBuildAlloca(builder, data_type, var_name_cstr.as_ptr())
    };
    if let Some(value) = initial_value {
        unsafe {
            LLVMBuildStore(builder, value, alloca);
        }
    }
    alloca
}

/// Reassigns a variable
pub fn reassign_var(
    builder: LLVMBuilderRef,
    variable_alloc: LLVMValueRef,
    new_value: LLVMValueRef,
) {
    unsafe {
        LLVMBuildStore(builder, new_value, variable_alloc);
    }
}

/// Gets a variable
pub fn get_var(
    builder: LLVMBuilderRef,
    variable_type: LLVMTypeRef, 
    variable_alloc: LLVMValueRef, 
) -> LLVMValueRef {
    unsafe {
        LLVMBuildLoad2(builder, variable_type, variable_alloc, CString::new("tmpload").unwrap().as_ptr()) // Ignore warning
    }
}