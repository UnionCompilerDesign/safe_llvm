extern crate llvm_sys as llvm;

use std::ffi::CString;

use llvm::{
        core::{
            LLVMBuildAlloca, LLVMBuildStore, LLVMBuildLoad2,
        }, 
        prelude::{
            LLVMBuilderRef, LLVMTypeRef, LLVMValueRef,
        }
};
use crate::memory_management::ir_pointer::IRPointer;

/// Initializes a variable
pub fn init_var(builder: LLVMBuilderRef, var_name: &str, data_type: LLVMTypeRef, initial_value: Option<LLVMValueRef>) -> IRPointer<LLVMValueRef> {
    let var_name_cstr = CString::new(var_name).unwrap();
    let alloca = unsafe {
        LLVMBuildAlloca(builder, data_type, var_name_cstr.as_ptr())
    };
    if let Some(value) = initial_value {
        unsafe {
            LLVMBuildStore(builder, value, alloca);
        }
    }
    IRPointer::new(Some(alloca as *mut _))
}

/// Reassigns a variable
pub fn reassign_var(builder: LLVMBuilderRef, variable_alloc: LLVMValueRef, new_value: LLVMValueRef) {
    unsafe {
        LLVMBuildStore(builder, new_value, variable_alloc);
    }
}

/// Gets a variable
pub fn get_var(builder: LLVMBuilderRef, variable_type: LLVMTypeRef, variable_alloc: LLVMValueRef) -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        LLVMBuildLoad2(builder, variable_type, variable_alloc, CString::new("tmpload").unwrap().as_ptr())
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}
