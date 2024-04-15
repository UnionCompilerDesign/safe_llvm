extern crate llvm_sys as llvm;

use llvm::{core, prelude::*};
use std::ffi::CString;
use crate::memory_management::ir_pointer::IRPointer;

/// Creates an integer
pub fn create_integer(val: i64, context: LLVMContextRef) -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        core::LLVMConstInt(
            core::LLVMInt64TypeInContext(context),
            val as u64,
            0 // isSigned flag
        )
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Creates a float
pub fn create_float(val: f64, context: LLVMContextRef) -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        core::LLVMConstReal(core::LLVMDoubleTypeInContext(context), val)
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Creates a boolean
pub fn create_boolean(val: bool, context: LLVMContextRef) -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        core::LLVMConstInt(core::LLVMInt1TypeInContext(context), val as u64, 0)
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Creates a function type
pub fn create_function(name: &str, return_type: Option<LLVMTypeRef>, param_types: &[LLVMTypeRef], 
    is_var_arg: bool, module: LLVMModuleRef) -> IRPointer<LLVMValueRef> {
    let llvm_return_type = match return_type {
        Some(ty) => ty,
        None => unsafe { core::LLVMVoidTypeInContext(core::LLVMGetModuleContext(module)) },
    };

    let function_type = unsafe {
        core::LLVMFunctionType(llvm_return_type, param_types.as_ptr() as *mut _, param_types.len() as u32, is_var_arg as i32)
    };
    let c_name = CString::new(name).expect("Failed to create function name");
    let raw_ptr = unsafe {
        core::LLVMAddFunction(module, c_name.as_ptr(), function_type)
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Creates an array
pub fn create_array(value: LLVMValueRef, num_elements: u64) -> IRPointer<LLVMValueRef> {
    let values = vec![value; num_elements as usize];
    let raw_ptr = unsafe {
        core::LLVMConstArray2(core::LLVMTypeOf(value), values.as_ptr() as *mut _, num_elements)
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Creates a pointer
pub fn create_pointer(value: LLVMValueRef) -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        core::LLVMConstPointerNull(core::LLVMPointerType(core::LLVMTypeOf(value), 0))
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Creates a struct
pub fn create_struct(values: &[LLVMValueRef], context: LLVMContextRef, packed: bool) -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        core::LLVMConstStructInContext(context, values.as_ptr() as *mut _, values.len() as u32, packed as i32)
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Creates a global variable
pub fn create_global_variable(module: LLVMModuleRef, initializer: LLVMValueRef, name: &str) -> IRPointer<LLVMValueRef> {
    let c_name = CString::new(name).expect("Failed to create global variable name");
    let raw_ptr = unsafe {
        let global_var = core::LLVMAddGlobal(module, core::LLVMTypeOf(initializer), c_name.as_ptr());
        core::LLVMSetInitializer(global_var, initializer);
        global_var
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Creates an immutable (global) string
pub fn create_string(val: &str, builder: LLVMBuilderRef) -> IRPointer<LLVMValueRef> {
    let c_val = CString::new(val).expect("Failed to create string");
    let c_str_name = CString::new("const_str").expect("Failed to create string name");
    let raw_ptr = unsafe {
        core::LLVMBuildGlobalStringPtr(builder, c_val.as_ptr(), c_str_name.as_ptr())
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Creates a mutable (local) string
pub fn create_mut_string(val: &str, context: LLVMContextRef, builder: LLVMBuilderRef) -> IRPointer<LLVMValueRef> {
    let c_str_name: CString = CString::new("local_str").expect("Failed to create string name");
    let raw_ptr = unsafe {
        let i8_type: LLVMTypeRef = core::LLVMInt8TypeInContext(context);
        let str_type: LLVMTypeRef = core::LLVMArrayType2(i8_type, val.len() as u64);
        let local_str: LLVMValueRef = core::LLVMBuildAlloca(builder, str_type, c_str_name.as_ptr());

        for (i, &byte) in val.as_bytes().iter().enumerate() {
            let index: LLVMValueRef = core::LLVMConstInt(core::LLVMInt32TypeInContext(context), i as u64, 0);
            let mut indices: [LLVMValueRef; 1] = [index];
            let gep: LLVMValueRef = core::LLVMBuildGEP2(builder, str_type, local_str, indices.as_mut_ptr(), indices.len() as u32, c_str_name.as_ptr());
            core::LLVMBuildStore(builder, core::LLVMConstInt(i8_type, byte as u64, 0), gep);
        }

        local_str
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Creates a null pointer
pub fn create_null_pointer(ty: LLVMTypeRef) -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        core::LLVMConstPointerNull(ty)
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// Creates a continue statement
pub fn create_continue_statement(builder: LLVMBuilderRef, continue_block: LLVMBasicBlockRef) -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        core::LLVMBuildBr(builder, continue_block)
    };
    IRPointer::new(Some(raw_ptr as *mut _))  
}

/// Creates a break statement
pub fn create_break_statement(builder: LLVMBuilderRef, break_block: LLVMBasicBlockRef) -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        core::LLVMBuildBr(builder, break_block)
    };
    IRPointer::new(Some(raw_ptr as *mut _))  
}

/// Creates a function type
pub fn create_function_type(return_type: LLVMTypeRef, param_types: &[LLVMTypeRef], is_var_arg: bool) -> IRPointer<LLVMTypeRef> {
    let raw_ptr = unsafe {
        core::LLVMFunctionType(
            return_type,
            param_types.as_ptr() as *mut LLVMTypeRef,
            param_types.len() as u32,
            is_var_arg as i32
        )
    };
    IRPointer::new(Some(raw_ptr as *mut _))  
}