extern crate llvm_sys as llvm;

use llvm::{core, prelude::*};
use crate::memory_management::pointer::CPointer;

/// void type
pub fn void_type(context: CPointer<LLVMContextRef>) -> CPointer<LLVMTypeRef> {
    let context_ptr = context.get_ref();
    let raw_ptr = unsafe {
        core::LLVMVoidTypeInContext(*context_ptr)
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// integer type
pub fn int_type(context: CPointer<LLVMContextRef>) -> CPointer<LLVMTypeRef> {
    let context_ptr = context.get_ref();
    let raw_ptr = unsafe {
        core::LLVMIntTypeInContext(*context_ptr, 64)
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// float type
pub fn float_type(context: CPointer<LLVMContextRef>) -> CPointer<LLVMTypeRef> {
    let context_ptr = context.get_ref();
    let raw_ptr = unsafe {
        core::LLVMFloatTypeInContext(*context_ptr)
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// boolean type
pub fn boolean_type(context: CPointer<LLVMContextRef>) -> CPointer<LLVMTypeRef> {
    let context_ptr = context.get_ref();
    let raw_ptr = unsafe {
        core::LLVMInt1TypeInContext(*context_ptr)
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// pointer type
pub fn pointer_type(element_type: CPointer<LLVMTypeRef>) -> CPointer<LLVMTypeRef> {
    let element_type_ptr = element_type.get_ref();
    let raw_ptr = unsafe {
        core::LLVMPointerType(*element_type_ptr, 0)
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// array type
pub fn array_type(element_type: CPointer<LLVMTypeRef>, num_elements: u64) -> CPointer<LLVMTypeRef> {
    let element_type_ptr = element_type.get_ref();
    let raw_ptr = unsafe {
        core::LLVMArrayType2(*element_type_ptr, num_elements)
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// struct type
pub fn struct_type(context: CPointer<LLVMContextRef>, element_types: &[CPointer<LLVMTypeRef>], packed: bool) -> CPointer<LLVMTypeRef> {
    let context_ptr = context.get_ref();
    let mut raw_element_types: Vec<*mut LLVMTypeRef> = element_types.iter().map(|et| et.get_ref()).collect();
    let raw_ptr = unsafe {
        core::LLVMStructTypeInContext(*context_ptr, *raw_element_types.as_mut_ptr(), raw_element_types.len() as u32, packed as i32)
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// returns nothing
pub fn void_return(builder: CPointer<LLVMBuilderRef>) -> CPointer<LLVMValueRef> {
    let builder_ptr = builder.get_ref();
    let raw_ptr = unsafe {
        core::LLVMBuildRetVoid(*builder_ptr)
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}

/// returns something
pub fn nonvoid_return(builder: CPointer<LLVMBuilderRef>, value: CPointer<LLVMValueRef>) -> CPointer<LLVMValueRef> {
    let builder_ptr = builder.get_ref();
    let value_ptr = value.get_ref();
    let raw_ptr = unsafe {
        core::LLVMBuildRet(*builder_ptr, *value_ptr)
    };
    let c_pointer = CPointer::new(raw_ptr as *mut _);
    if c_pointer.is_some() {
        return c_pointer.unwrap();
    }
    panic!("Missing c_pointer")}
