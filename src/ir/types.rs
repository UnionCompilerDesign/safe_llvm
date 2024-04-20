extern crate llvm_sys as llvm;

use llvm::{core, prelude::*};
use crate::memory_management::pointer::CPointer;

/// void type
pub fn void_type(context: CPointer<LLVMContextRef>) -> CPointer<LLVMTypeRef> {
    let context_ptr = context.get_ref();
    let raw_ptr = unsafe {
        core::LLVMVoidTypeInContext(*context_ptr)
    };
    CPointer::new(Some(raw_ptr as *mut _))
}

/// integer type
pub fn int_type(context: CPointer<LLVMContextRef>) -> CPointer<LLVMTypeRef> {
    let context_ptr = context.get_ref();
    let raw_ptr = unsafe {
        core::LLVMIntTypeInContext(*context_ptr, 64)
    };
    CPointer::new(Some(raw_ptr as *mut _))
}

/// float type
pub fn float_type(context: CPointer<LLVMContextRef>) -> CPointer<LLVMTypeRef> {
    let context_ptr = context.get_ref();
    let raw_ptr = unsafe {
        core::LLVMFloatTypeInContext(*context_ptr)
    };
    CPointer::new(Some(raw_ptr as *mut _))
}

/// boolean type
pub fn boolean_type(context: CPointer<LLVMContextRef>) -> CPointer<LLVMTypeRef> {
    let context_ptr = context.get_ref();
    let raw_ptr = unsafe {
        core::LLVMInt1TypeInContext(*context_ptr)
    };
    CPointer::new(Some(raw_ptr as *mut _))
}

/// pointer type
pub fn pointer_type(element_type: CPointer<LLVMTypeRef>) -> CPointer<LLVMTypeRef> {
    let element_type_ptr = element_type.get_ref();
    let raw_ptr = unsafe {
        core::LLVMPointerType(*element_type_ptr, 0)
    };
    CPointer::new(Some(raw_ptr as *mut _))
}

/// array type
pub fn array_type(element_type: CPointer<LLVMTypeRef>, num_elements: u64) -> CPointer<LLVMTypeRef> {
    let element_type_ptr = element_type.get_ref();
    let raw_ptr = unsafe {
        core::LLVMArrayType2(*element_type_ptr, num_elements)
    };
    CPointer::new(Some(raw_ptr as *mut _))
}

/// struct type
pub fn struct_type(context: CPointer<LLVMContextRef>, element_types: &[CPointer<LLVMTypeRef>], packed: bool) -> CPointer<LLVMTypeRef> {
    let context_ptr = context.get_ref();
    let mut raw_element_types: Vec<*mut LLVMTypeRef> = element_types.iter().map(|et| et.get_ref()).collect();
    let raw_ptr = unsafe {
        core::LLVMStructTypeInContext(*context_ptr, *raw_element_types.as_mut_ptr(), raw_element_types.len() as u32, packed as i32)
    };
    CPointer::new(Some(raw_ptr as *mut _))
}

/// returns nothing
pub fn void_return(builder: CPointer<LLVMBuilderRef>) -> CPointer<LLVMValueRef> {
    let builder_ptr = builder.get_ref();
    let raw_ptr = unsafe {
        core::LLVMBuildRetVoid(*builder_ptr)
    };
    CPointer::new(Some(raw_ptr as *mut _))
}

/// returns something
pub fn nonvoid_return(builder: CPointer<LLVMBuilderRef>, value: CPointer<LLVMValueRef>) -> CPointer<LLVMValueRef> {
    let builder_ptr = builder.get_ref();
    let value_ptr = value.get_ref();
    let raw_ptr = unsafe {
        core::LLVMBuildRet(*builder_ptr, *value_ptr)
    };
    CPointer::new(Some(raw_ptr as *mut _))
}
