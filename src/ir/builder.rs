extern crate llvm_sys as llvm;

use std::{ffi::{c_uint, CString}, ptr};

use llvm::{core::{self, LLVMAddFunction, LLVMFunctionType, LLVMGetModuleContext, LLVMVoidTypeInContext}, prelude::{LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef}};

use crate::memory_management::pointer::CPointer; 

/// Creates a builder in the specified LLVM context
pub fn create_builder(context: CPointer<LLVMContextRef>) -> CPointer<LLVMBuilderRef> {
    let context_ptr: *mut LLVMContextRef = context.get_ref();

    let raw_ptr = unsafe {
        core::LLVMCreateBuilderInContext(*context_ptr)
    };

    CPointer::new(Some(raw_ptr as *mut _))
}

/// Creates a new function within the given LLVM module.
pub fn create_function(
    name: &str,
    return_type: Option<CPointer<LLVMTypeRef>>,
    param_types: &[CPointer<LLVMTypeRef>],
    is_var_arg: bool,
    module: CPointer<LLVMModuleRef>
) -> CPointer<LLVMValueRef> {
    unsafe {
        let module_ptr = module.get_ref();

        let llvm_return_type = match return_type {
            Some(ref_type) => *ref_type.get_ref(),
            None => LLVMVoidTypeInContext(LLVMGetModuleContext(*module_ptr)),
        };

        let llvm_param_types: Vec<*mut LLVMTypeRef> = param_types.iter().map(|ty| {
            ty.get_ref()  
        }).collect();

        let function_type = LLVMFunctionType(
            llvm_return_type,
            llvm_param_types.as_ptr() as *mut _,
            llvm_param_types.len() as c_uint,
            is_var_arg as i32,
        );

        let c_name = CString::new(name).expect("Failed to create function name");

        let raw_ptr = LLVMAddFunction(*module_ptr, c_name.as_ptr(), function_type);

        CPointer::new(Some(raw_ptr as *mut _))
    }
}