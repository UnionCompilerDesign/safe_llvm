extern crate llvm_sys as llvm;

use llvm::{core, prelude::LLVMValueRef, LLVMBasicBlock, LLVMBuilder, LLVMContext, LLVMModule, LLVMType, LLVMValue};

use std::{ffi::CString, sync::{Arc, Mutex}};

use crate::memory_management::resource_pools::{Handle, LLVMResourcePools};

/// Creates an integer
pub fn create_integer(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, val: i64, context_handle: Handle) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let context = pool_guard.get_context(context_handle)?;
    drop(pool_guard);

    let integer_type = unsafe {
        context.read().unwrap().use_ref(|context_ptr| {
            core::LLVMConstInt(
                core::LLVMInt64TypeInContext(context_ptr),
                val as u64,
                0 // isSigned flag
            )
        })
    };

    if integer_type.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(integer_type)
    }
}

/// Creates a float
pub fn create_float(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, val: f64, context_handle: Handle) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let context = pool_guard.get_context(context_handle)?;
    drop(pool_guard);

    let float_type = unsafe {
        context.read().unwrap().use_ref(|context_ptr| {
            core::LLVMConstReal(core::LLVMDoubleTypeInContext(context_ptr), val)
        })
    };

    if float_type.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(float_type)
    }
}

/// Creates a boolean
pub fn create_boolean(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, val: bool, context_handle: Handle) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let context = pool_guard.get_context(context_handle)?;
    drop(pool_guard);

    let boolean_type = unsafe {
        context.read().unwrap().use_ref(|context_ptr| {
            core::LLVMConstInt(core::LLVMInt1TypeInContext(context_ptr), val as u64, 0)
        })
    };

    if boolean_type.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(boolean_type)
    }
}


/// Creates an array
pub fn create_array(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, value_handle: Handle, num_elements: u64) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let value = pool_guard.get_value(value_handle)?;
    drop(pool_guard);

    let array_type = unsafe {
        value.read().unwrap().use_ref(|value_ptr| {
            let values = vec![value_ptr; num_elements as usize];
            core::LLVMConstArray2(core::LLVMTypeOf(value_ptr), values.as_ptr() as *mut _, num_elements)
        })
    };

    if array_type.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(array_type)
    }
}

/// Creates a pointer
pub fn create_pointer(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, element_type_handle: Handle) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let element_type = pool_guard.get_type(element_type_handle)?;
    drop(pool_guard);

    let pointer_type = unsafe {
        element_type.read().unwrap().use_ref(|element_type_ptr| {
            core::LLVMConstPointerNull(core::LLVMPointerType(element_type_ptr, 0))
        })
    };

    if pointer_type.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(pointer_type)
    }
}

/// Creates a struct
pub fn create_struct(pool: Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, values: &[Handle], context_handle: Handle, packed: bool) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let context = pool_guard.get_context(context_handle)?;
    let value_ptrs: Vec<LLVMValueRef> = values.iter().map(|&handle| pool_guard.get_value(handle).unwrap().read().unwrap().use_ref(|ptr| ptr)).collect();
    drop(pool_guard);

    let struct_type = unsafe {
        context.read().unwrap().use_ref(|context_ptr| {
            core::LLVMConstStructInContext(context_ptr, value_ptrs.as_ptr() as *mut _, value_ptrs.len() as u32, packed as i32)
        })
    };

    if struct_type.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(struct_type)
    }
}


/// Creates a global variable
pub fn create_global_variable(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, module_handle: Handle, initializer_handle: Handle, name: &str) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let module = pool_guard.get_module(module_handle)?;
    let initializer = pool_guard.get_value(initializer_handle)?;
    drop(pool_guard);

    let c_name = CString::new(name).expect("Failed to create global variable name");

    let global_var = unsafe {
        module.read().unwrap().use_ref(|module_ptr| {
            initializer.read().unwrap().use_ref(|initializer_ptr| {
                let global_var = core::LLVMAddGlobal(module_ptr, core::LLVMTypeOf(initializer_ptr), c_name.as_ptr());
                core::LLVMSetInitializer(global_var, initializer_ptr);
                global_var
            })
        })
    };

    if global_var.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(global_var)
    }
}


/// Creates an immutable (global) string
pub fn create_string(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, val: &str, builder_handle: Handle) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    drop(pool_guard);

    let c_val = CString::new(val).expect("Failed to create string");
    let c_str_name = CString::new("const_str").expect("Failed to create string name");

    let str_pointer = unsafe {
        builder.read().unwrap().use_ref(|builder_ptr| {
            core::LLVMBuildGlobalStringPtr(builder_ptr, c_val.as_ptr(), c_str_name.as_ptr())
        })
    };

    if str_pointer.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(str_pointer)
    }
}

/// Creates a mutable (local) string
pub fn create_mut_string(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, val: &str, context_handle: Handle, builder_handle: Handle) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let context = pool_guard.get_context(context_handle)?;
    let builder = pool_guard.get_builder(builder_handle)?;
    drop(pool_guard);

    let c_str_name = CString::new("local_str").expect("Failed to create string name");

    let local_str = unsafe {
        context.read().unwrap().use_ref(|context_ptr| {
            builder.read().unwrap().use_ref(|builder_ptr| {
                let i8_type = core::LLVMInt8TypeInContext(context_ptr);
                let str_type = core::LLVMArrayType2(i8_type, val.len() as u64);
                let local_str = core::LLVMBuildAlloca(builder_ptr, str_type, c_str_name.as_ptr());

                for (i, &byte) in val.as_bytes().iter().enumerate() {
                    let index = core::LLVMConstInt(core::LLVMInt32TypeInContext(context_ptr), i as u64, 0);
                    let mut indices: [LLVMValueRef; 1] = [index];
                    let gep = core::LLVMBuildGEP2(builder_ptr, str_type, local_str, indices.as_mut_ptr(), indices.len() as u32, c_str_name.as_ptr());
                    core::LLVMBuildStore(builder_ptr, core::LLVMConstInt(i8_type, byte as u64, 0), gep);
                }

                local_str
            })
        })
    };

    if local_str.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(local_str)
    }
}

/// Creates a null pointer
pub fn create_null_pointer(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, ty_handle: Handle) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let ty = pool_guard.get_type(ty_handle)?;
    drop(pool_guard);

    let null_pointer = unsafe {
        ty.read().unwrap().use_ref(|ty_ptr| {
            core::LLVMConstPointerNull(ty_ptr)
        })
    };

    if null_pointer.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(null_pointer)
    }
}

/// Creates a continue statement
pub fn create_continue_statement(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, builder_handle: Handle, continue_block_handle: Handle) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    let continue_block = pool_guard.get_basic_block(continue_block_handle)?;
    drop(pool_guard);

    let continue_statement = unsafe {
        builder.read().unwrap().use_ref(|builder_ptr| {
            continue_block.read().unwrap().use_ref(|continue_block_ptr| {
                core::LLVMBuildBr(builder_ptr, continue_block_ptr)
            })
        })
    };

    if continue_statement.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(continue_statement)
    }
}

/// Creates a break statement
pub fn create_break_statement(pool: &Arc<Mutex<LLVMResourcePools<LLVMContext, LLVMModule, LLVMValue, LLVMBasicBlock, LLVMBuilder, LLVMType>>>, builder_handle: Handle, break_block_handle: Handle) -> Option<Handle> {
    let pool_guard = pool.lock().unwrap();
    let builder = pool_guard.get_builder(builder_handle)?;
    let break_block = pool_guard.get_basic_block(break_block_handle)?;
    drop(pool_guard);

    let break_statement = unsafe {
        builder.read().unwrap().use_ref(|builder_ptr| {
            break_block.read().unwrap().use_ref(|break_block_ptr| {
                core::LLVMBuildBr(builder_ptr, break_block_ptr)
            })
        })
    };

    if break_statement.is_null() {
        None
    } else {
        let mut pool_guard = pool.lock().unwrap();
        pool_guard.create_value(break_statement)
    }
}