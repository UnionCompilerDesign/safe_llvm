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

    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Creates a new function within the given LLVM module.
pub fn create_function(
    name: &str,
    return_type: Option<CPointer<LLVMTypeRef>>,
    param_types: &[CPointer<LLVMTypeRef>],
    is_var_arg: bool,
    module: CPointer<LLVMModuleRef>
) -> Option<CPointer<LLVMValueRef>> {
    unsafe {
        let module_ptr = module.get_ref();
        if module_ptr.is_null() {
            panic!("Module pointer is null");
        }

        let llvm_return_type = match return_type {
            Some(ref_type) => {
                let rt = ref_type.get_ref();
                if rt.is_null() {
                    panic!("Return type pointer is null");
                }
                *rt
            },
            None => {
                let context = LLVMGetModuleContext(*module_ptr);
                if context.is_null() {
                    panic!("Failed to obtain a valid LLVM context from the module");
                }
                LLVMVoidTypeInContext(context)
            },
        };

        let llvm_param_types: Vec<LLVMTypeRef> = param_types.iter().map(|ty| {
            let ty_ptr = ty.get_ref();
            if ty_ptr.is_null() {
                panic!("One of the parameter type pointers is null");
            }
            *ty_ptr
        }).collect();

        if llvm_param_types.is_empty() && !param_types.is_empty() {
            panic!("Failed to properly convert parameter types");
        }

        let param_ptr = llvm_param_types.as_ptr() as *mut LLVMTypeRef;
        let param_count = llvm_param_types.len() as u32;

        let function_type = LLVMFunctionType(llvm_return_type, param_ptr, param_count, is_var_arg as i32);
        if function_type.is_null() {
            panic!("Failed to create LLVM function type");
        }

        let c_name = CString::new(name).expect("Failed to create function name due to null bytes.");

        let raw_ptr = LLVMAddFunction(*module_ptr, c_name.as_ptr(), function_type);
        if raw_ptr.is_null() {
            panic!("Failed to add function to LLVM module");
        }

        CPointer::new(raw_ptr as *mut _)
    }
}