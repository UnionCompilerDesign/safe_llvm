use llvm::{
    core, 
    prelude::*,
    LLVMValue,
    LLVMType,
}; // change to not use wild star import
use std::ffi::CString;

/// creates an integer
pub fn create_integer(val: i64, context: LLVMContextRef) -> LLVMValueRef {
    unsafe {
        core::LLVMConstInt(
            core::LLVMInt64TypeInContext(context),
            val as u64,
            0 // isSigned flag
        )
    }
}

/// creates a float
pub fn create_float(val: f64, context: LLVMContextRef) -> LLVMValueRef {
    unsafe {
        core::LLVMConstReal(core::LLVMDoubleTypeInContext(context), val)
    }
}

/// creates a boolean
pub fn create_boolean(val: bool, context: LLVMContextRef) -> LLVMValueRef {
    unsafe {
        core::LLVMConstInt(core::LLVMInt1TypeInContext(context), val as u64, 0)
    }
}

/// creates a function type
pub fn create_function(name: &str, return_type: Option<LLVMTypeRef>, param_types: &[LLVMTypeRef], 
    is_var_arg: bool, module: LLVMModuleRef) -> LLVMValueRef {
    let llvm_return_type = match return_type {
        Some(ty) => ty,
        None => unsafe { core::LLVMVoidTypeInContext(core::LLVMGetModuleContext(module)) },
    };

    let function_type = unsafe {
        core::LLVMFunctionType(llvm_return_type, param_types.as_ptr() as *mut _, param_types.len() as u32, is_var_arg as i32)
    };
    let c_name = CString::new(name).expect("Failed to create function name");
    unsafe {
        core::LLVMAddFunction(module, c_name.as_ptr(), function_type)
    }
}

/// creates an array
pub fn create_array(value: LLVMValueRef, num_elements: u64) -> LLVMValueRef {
    let values = vec![value; num_elements as usize];
    unsafe {
        core::LLVMConstArray2(core::LLVMTypeOf(value), values.as_ptr() as *mut _, num_elements)
    }
}

/// creates a pointer
pub fn create_pointer(value: LLVMValueRef) -> LLVMValueRef {
    unsafe {
        core::LLVMConstPointerNull(core::LLVMPointerType(core::LLVMTypeOf(value), 0))
    }
}

/// creates a struct
pub fn create_struct(values: &[LLVMValueRef], context: LLVMContextRef, packed: bool) -> LLVMValueRef {
    unsafe {
        core::LLVMConstStructInContext(context, values.as_ptr() as *mut _, values.len() as u32, packed as i32)
    }
}

/// creates a global variable
pub fn create_global_variable(module: LLVMModuleRef, initializer: LLVMValueRef, name: &str) -> LLVMValueRef {
    let c_name = CString::new(name).expect("Failed to create global variable name");
    unsafe {
        let global_var = core::LLVMAddGlobal(module, core::LLVMTypeOf(initializer), c_name.as_ptr());
        core::LLVMSetInitializer(global_var, initializer);
        global_var
    }
}

/// creates immutable (global) string
pub fn create_string(val: &str, builder: LLVMBuilderRef) -> LLVMValueRef {
    let c_val = CString::new(val).expect("Failed to create string");
    let c_str_name = CString::new("const_str").expect("Failed to create string name");
    unsafe {
        core::LLVMBuildGlobalStringPtr(builder, c_val.as_ptr(), c_str_name.as_ptr())
    }
}

/// creates a mutable (local) string
pub fn create_mut_string(val: &str, context: LLVMContextRef, builder: LLVMBuilderRef) -> LLVMValueRef {
    let c_str_name: CString = CString::new("local_str").expect("Failed to create string name");
    unsafe {
        let i8_type: *mut LLVMType = core::LLVMInt8TypeInContext(context);
        let str_type: *mut LLVMType = core::LLVMArrayType2(i8_type, val.len() as u64);
        let local_str: *mut LLVMValue = core::LLVMBuildAlloca(builder, str_type, c_str_name.as_ptr());

        for (i, &byte) in val.as_bytes().iter().enumerate() {
            let index: *mut LLVMValue = core::LLVMConstInt(core::LLVMInt32TypeInContext(context), i as u64, 0);
            let mut indices: [*mut LLVMValue; 1] = [index];
            let gep: *mut LLVMValue = core::LLVMBuildGEP2(builder, str_type, local_str, indices.as_mut_ptr(), indices.len().try_into().unwrap(), c_str_name.as_ptr());
            core::LLVMBuildStore(builder, core::LLVMConstInt(i8_type, byte as u64, 0), gep);
        }

        local_str
    }
}

/// creates a null pointer
pub fn create_null_pointer(ty: LLVMTypeRef) -> LLVMValueRef {
    unsafe {
        core::LLVMConstPointerNull(ty)
    }
}

/// creates a continue statement
pub fn create_continue_statement(builder: LLVMBuilderRef, continue_block: LLVMBasicBlockRef) {
    unsafe {
        core::LLVMBuildBr(builder, continue_block);
    }
}

/// creates a break statement
pub fn create_break_statement(builder: LLVMBuilderRef, break_block: LLVMBasicBlockRef) {
    unsafe {
        core::LLVMBuildBr(builder, break_block);
    }
}

/// creates a function
pub fn create_function_type(return_type: LLVMTypeRef, param_types: &[LLVMTypeRef], is_var_arg: bool) -> LLVMTypeRef {
    unsafe {
        core::LLVMFunctionType(
            return_type,
            param_types.as_ptr() as *mut LLVMTypeRef,
            param_types.len() as u32,
            is_var_arg as i32, 
        )
    }
}

/// adds a function to a module
pub fn add_function_to_module(module: LLVMModuleRef, function_name: &str, function_type: LLVMTypeRef) -> LLVMValueRef {
    let c_name = CString::new(function_name).expect("Failed to create CString for function name");
    unsafe {
        core::LLVMAddFunction(module, c_name.as_ptr(), function_type)
    }
}

/// creates a conditional branch
pub fn create_cond_br(builder: LLVMBuilderRef, condition: LLVMValueRef, then_bb: LLVMBasicBlockRef, else_bb: LLVMBasicBlockRef) -> LLVMValueRef {
    unsafe {
        core::LLVMBuildCondBr(builder, condition, then_bb, else_bb)
    }
}

/// creates an unconditional branch
pub fn create_br(builder: LLVMBuilderRef, target_bb: LLVMBasicBlockRef) -> LLVMValueRef {
    unsafe {
        core::LLVMBuildBr(builder, target_bb)
    }
}