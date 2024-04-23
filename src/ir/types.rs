extern crate llvm_sys as llvm;

use llvm::{core, prelude::LLVMTypeRef, LLVMBasicBlock, LLVMBuilder, LLVMContext, LLVMModule, LLVMType, LLVMValue};

use std::sync::{Arc, Mutex};

use crate::memory_management::resource_pools::{Handle, ResourcePools};

/// void type
pub fn void_type(pool: &Arc<Mutex<ResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, context_handle: Handle) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let context = pool_guard.get_context(context_handle)?;
    drop(pool_guard);

    let void_type = unsafe {
        context.read().unwrap().use_ref(|context_ptr| {
            core::LLVMVoidTypeInContext(context_ptr)
        })
    };

    if void_type.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_type(void_type)
    }
}

/// integer type
pub fn int_type(pool: &Arc<Mutex<ResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, context_handle: Handle) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let context = pool_guard.get_context(context_handle)?;
    drop(pool_guard);

    let int_type = unsafe {
        context.read().unwrap().use_ref(|context_ptr| {
            core::LLVMIntTypeInContext(context_ptr, 64)
        })
    };

    if int_type.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_type(int_type)
    }
}

/// float type
pub fn float_type(pool: &Arc<Mutex<ResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, context_handle: Handle) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let context = pool_guard.get_context(context_handle)?;
    drop(pool_guard);

    let float_type = unsafe {
        context.read().unwrap().use_ref(|context_ptr| {
            core::LLVMFloatTypeInContext(context_ptr)
        })
    };

    if float_type.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_type(float_type)
    }
}

/// boolean type
pub fn boolean_type(pool: &Arc<Mutex<ResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, context_handle: Handle) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let context = pool_guard.get_context(context_handle)?;
    drop(pool_guard);

    let boolean_type = unsafe {
        context.read().unwrap().use_ref(|context_ptr| {
            core::LLVMInt1TypeInContext(context_ptr)
        })
    };

    if boolean_type.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_type(boolean_type)
    }
}

/// pointer type
pub fn pointer_type(pool: &Arc<Mutex<ResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, element_type_handle: Handle) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let element_type = pool_guard.get_type(element_type_handle)?;
    drop(pool_guard);

    let pointer_type = unsafe {
        element_type.read().unwrap().use_ref(|element_type_ptr| {
            core::LLVMPointerType(element_type_ptr, 0)
        })
    };

    if pointer_type.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_type(pointer_type)
    }
}

/// array type
pub fn array_type(pool: &Arc<Mutex<ResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, element_type_handle: Handle, num_elements: u64) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let element_type = pool_guard.get_type(element_type_handle)?;
    drop(pool_guard);

    let array_type = unsafe {
        element_type.read().unwrap().use_ref(|element_type_ptr| {
            core::LLVMArrayType2(element_type_ptr, num_elements)
        })
    };

    if array_type.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_type(array_type)
    }
}

/// struct type
pub fn struct_type(pool: &Arc<Mutex<ResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, context_handle: Handle, element_type_handles: &[Handle], packed: bool) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let context = pool_guard.get_context(context_handle)?;
    let mut element_types: Vec<LLVMTypeRef> = element_type_handles.iter().map(|&handle| pool_guard.get_type(handle).unwrap().read().unwrap().use_ref(|ptr| ptr)).collect();
    drop(pool_guard);

    let struct_type = unsafe {
        context.read().unwrap().use_ref(|context_ptr| {
            core::LLVMStructTypeInContext(context_ptr, element_types.as_mut_ptr(), element_types.len() as u32, packed as i32)
        })
    };

    if struct_type.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_type(struct_type)
    }
}

/// returns nothing
pub fn void_return(pool: &Arc<Mutex<ResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, builder_handle: Handle) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    drop(pool_guard);

    let void_return_inst = unsafe {
        builder.read().unwrap().use_ref(|builder_ptr| {
            core::LLVMBuildRetVoid(builder_ptr)
        })
    };

    if void_return_inst.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(void_return_inst)
    }
}

/// returns something
pub fn nonvoid_return(pool: &Arc<Mutex<ResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, builder_handle: Handle, value_handle: Handle) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    let value = pool_guard.get_value(value_handle)?;
    drop(pool_guard);

    let nonvoid_return_inst = unsafe {
        builder.read().unwrap().use_ref(|builder_ptr| {
            value.read().unwrap().use_ref(|value_ptr| {
                core::LLVMBuildRet(builder_ptr, value_ptr)
            })
        })
    };

    if nonvoid_return_inst.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(nonvoid_return_inst)
    }
}

