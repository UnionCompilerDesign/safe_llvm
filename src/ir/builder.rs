extern crate llvm_sys as llvm;

use llvm::{core, prelude::*, LLVMBasicBlock, LLVMBuilder, LLVMContext, LLVMModule, LLVMType, LLVMValue};

use std::sync::{Arc, Mutex};

use crate::memory_management::resource_pools::{LLVMResourcePools, Handle};

pub fn create_builder(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, context_handle: Handle) -> Option<Handle> {
    let pool_guard = pool.lock().expect("Failed to lock pool");
    let context_lock = pool_guard.get_context(context_handle)?;
    drop(pool_guard); 

    let builder_ptr = context_lock.read().expect("Failed to lock context").use_ref(|context_ptr| {
        unsafe { core::LLVMCreateBuilderInContext(context_ptr) }
    });

    let mut pool_guard = pool.lock().expect("Failed to re-lock pool");
    pool_guard.create_builder(builder_ptr)
}

pub fn create_function(
    pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>,
    return_type_handle: Option<Handle>,
    param_type_handles: &[Handle],
    is_var_arg: bool,
    context_handle: Handle,
) -> Option<Handle> {
    let pool_locked = pool.lock().expect("Failed to lock pool");

    let context_lock = pool_locked.get_context(context_handle)?;
    let context_ptr: LLVMContextRef = context_lock.read().unwrap().use_ref(|context| context);

    let llvm_return_type = match return_type_handle {
        Some(handle) => {
            let type_lock = pool_locked.get_type(handle)?;
            let ptr = type_lock.read().unwrap().use_ref(|type_ptr| type_ptr); 
            ptr
        },
        None => unsafe { core::LLVMVoidTypeInContext(context_ptr) },
    };

    let mut llvm_param_types: Vec<LLVMTypeRef> = Vec::new();
    for handle in param_type_handles {
        let type_lock = pool_locked.get_type(*handle)?;
        let type_ref: LLVMTypeRef = type_lock.read().unwrap().use_ref(|type_ptr| type_ptr);
        llvm_param_types.push(type_ref);
    }

    let param_ptr = if llvm_param_types.is_empty() {
        std::ptr::null_mut()
    } else {
        llvm_param_types.as_mut_ptr()
    };
    let param_count = llvm_param_types.len() as u32;

    let function_type = unsafe {
        core::LLVMFunctionType(llvm_return_type, param_ptr, param_count, is_var_arg as i32)
    };

    let mut pool_locked = pool.lock().expect("Failed to re-lock pool");
    pool_locked.create_type(function_type)
}