extern crate llvm_sys as llvm;

use llvm::{core, prelude::*};
use std::ffi::CString;
use crate::memory_management::pointer::CPointer;

/// Creates an integer
pub fn create_integer(val: i64, context: CPointer<LLVMContextRef>) -> CPointer<LLVMValueRef> {
    let context_ptr = context.get_ref();
    let raw_ptr = unsafe {
        core::LLVMConstInt(
            core::LLVMInt64TypeInContext(*context_ptr),
            val as u64,
            0 // isSigned flag
        )
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Creates a float
pub fn create_float(val: f64, context: CPointer<LLVMContextRef>) -> CPointer<LLVMValueRef> {
    let context_ptr = context.get_ref();
    let raw_ptr = unsafe {
        core::LLVMConstReal(core::LLVMDoubleTypeInContext(*context_ptr), val)
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Creates a boolean
pub fn create_boolean(val: bool, context: CPointer<LLVMContextRef>) -> CPointer<LLVMValueRef> {
    let context_ptr = context.get_ref();
    let raw_ptr = unsafe {
        core::LLVMConstInt(core::LLVMInt1TypeInContext(*context_ptr), val as u64, 0)
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}


/// Creates an array
pub fn create_array(value: CPointer<LLVMValueRef>, num_elements: u64) -> CPointer<LLVMValueRef> {
    let value_ptr = value.get_ref();
    let values = unsafe { vec![*value_ptr; num_elements as usize] };
    let raw_ptr = unsafe {
        core::LLVMConstArray2(core::LLVMTypeOf(*value_ptr), values.as_ptr() as *mut _, num_elements)
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Creates a pointer
pub fn create_pointer(value: CPointer<LLVMTypeRef>) -> CPointer<LLVMValueRef> {
    let value_ptr = value.get_ref();
    let raw_ptr = unsafe {
        core::LLVMConstPointerNull(core::LLVMPointerType(*value_ptr, 0))
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Creates a struct
pub fn create_struct(values: &[CPointer<LLVMValueRef>], context: CPointer<LLVMContextRef>, packed: bool) -> CPointer<LLVMValueRef> {
    let context_ptr = context.get_ref();
    let value_ptrs: Vec<*mut LLVMValueRef> = values.iter().map(|v| v.get_ref()).collect();
    let raw_ptr = unsafe {
        core::LLVMConstStructInContext(*context_ptr, value_ptrs.as_ptr() as *mut _, value_ptrs.len() as u32, packed as i32)
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Creates a global variable
pub fn create_global_variable(module: CPointer<LLVMModuleRef>, initializer: CPointer<LLVMValueRef>, name: &str) -> CPointer<LLVMValueRef> {
    let module_ptr = module.get_ref();
    let initializer_ptr = initializer.get_ref();
    let c_name = CString::new(name).expect("Failed to create global variable name");
    let raw_ptr = unsafe {
        let global_var = core::LLVMAddGlobal(*module_ptr, core::LLVMTypeOf(*initializer_ptr), c_name.as_ptr());
        core::LLVMSetInitializer(global_var, *initializer_ptr);
        global_var
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Creates an immutable (global) string
pub fn create_string(val: &str, builder: CPointer<LLVMBuilderRef>) -> CPointer<LLVMValueRef> {
    let builder_ptr = builder.get_ref();
    let c_val = CString::new(val).expect("Failed to create string");
    let c_str_name = CString::new("const_str").expect("Failed to create string name");
    let raw_ptr = unsafe {
        core::LLVMBuildGlobalStringPtr(*builder_ptr, c_val.as_ptr(), c_str_name.as_ptr())
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Creates a mutable (local) string
pub fn create_mut_string(val: &str, context: CPointer<LLVMContextRef>, builder: CPointer<LLVMBuilderRef>) -> CPointer<LLVMValueRef> {
    let context_ptr = context.get_ref();
    let builder_ptr = builder.get_ref();
    let c_str_name: CString = CString::new("local_str").expect("Failed to create string name");
    let raw_ptr = unsafe {
        let i8_type: LLVMTypeRef = core::LLVMInt8TypeInContext(*context_ptr);
        let str_type: LLVMTypeRef = core::LLVMArrayType2(i8_type, val.len() as u64);
        let local_str: LLVMValueRef = core::LLVMBuildAlloca(*builder_ptr, str_type, c_str_name.as_ptr());

        for (i, &byte) in val.as_bytes().iter().enumerate() {
            let index: LLVMValueRef = core::LLVMConstInt(core::LLVMInt32TypeInContext(*context_ptr), i as u64, 0);
            let mut indices: [LLVMValueRef; 1] = [index];
            let gep: LLVMValueRef = core::LLVMBuildGEP2(*builder_ptr, str_type, local_str, indices.as_mut_ptr(), indices.len() as u32, c_str_name.as_ptr());
            core::LLVMBuildStore(*builder_ptr, core::LLVMConstInt(i8_type, byte as u64, 0), gep);
        }

        local_str
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Creates a null pointer
pub fn create_null_pointer(ty: CPointer<LLVMTypeRef>) -> CPointer<LLVMValueRef> {
    let type_ptr = ty.get_ref();
    let raw_ptr = unsafe {
        core::LLVMConstPointerNull(*type_ptr)
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Creates a continue statement
pub fn create_continue_statement(builder: CPointer<LLVMBuilderRef>, continue_block: CPointer<LLVMBasicBlockRef>) -> CPointer<LLVMValueRef> {
    let builder_ptr = builder.get_ref();
    let continue_block_ptr = continue_block.get_ref();
    let raw_ptr = unsafe {
        core::LLVMBuildBr(*builder_ptr, *continue_block_ptr)
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// Creates a break statement
pub fn create_break_statement(builder: CPointer<LLVMBuilderRef>, break_block: CPointer<LLVMBasicBlockRef>) -> CPointer<LLVMValueRef> {
    let builder_ptr = builder.get_ref();
    let break_block_ptr = break_block.get_ref();
    let raw_ptr = unsafe {
        core::LLVMBuildBr(*builder_ptr, *break_block_ptr)
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")
}
