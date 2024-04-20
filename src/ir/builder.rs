extern crate llvm_sys as llvm;

use std::ffi::CString;

use llvm::{core::{self, LLVMAddFunction, LLVMFunctionType, LLVMGetModuleContext, LLVMVoidTypeInContext}, prelude::{LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef}};

use crate::memory_management::pointer::CPointer; 

/// Creates a builder in the specified LLVM context
pub fn create_builder(context: CPointer<LLVMContextRef>) -> CPointer<LLVMBuilderRef> {
    let context_ptr: *mut LLVMContextRef = context.get_ref();

    let raw_ptr = unsafe {
        core::LLVMCreateBuilderInContext(*context_ptr)
    };

    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Creates a new function type within the given LLVM context.
pub fn create_function(
    return_type: Option<CPointer<LLVMTypeRef>>,
    param_types: &[CPointer<LLVMTypeRef>],
    is_var_arg: bool,
    context: CPointer<LLVMContextRef>,
) -> Option<CPointer<LLVMTypeRef>> {
    unsafe {
        if context.is_null() {
            panic!("Context pointer is null or uninitialized");
        }

        let llvm_return_type = match return_type {
            Some(ref_type) => {
                if ref_type.is_null() {
                    panic!("Return type pointer is null or uninitialized");
                }
                *ref_type.get_ref()
            },
            None => LLVMVoidTypeInContext(*context.get_ref()),
        };

        let llvm_param_types: Vec<LLVMTypeRef> = param_types
            .iter()
            .map(|ty| {
                if ty.is_null() {
                    panic!("Parameter type pointer is null or uninitialized");
                }
                *ty.get_ref()
            })
            .collect();

        if llvm_param_types.is_empty() {
            panic!("No parameter types provided");
        }

        let param_ptr = llvm_param_types.as_ptr() as *mut LLVMTypeRef;
        let param_count = llvm_param_types.len() as u32;

        let function_type = LLVMFunctionType(llvm_return_type, param_ptr, param_count, is_var_arg as i32);
        if function_type.is_null() {
            panic!("Failed to create LLVM function type");
        }

        CPointer::new(function_type as *mut LLVMTypeRef)
    }
}
