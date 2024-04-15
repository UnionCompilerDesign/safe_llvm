extern crate llvm_sys as llvm;

use llvm::{core, prelude::{LLVMTypeRef, LLVMValueRef}};
use crate::memory_management::ir_pointer::IRPointer;

/// void type
pub fn void_type(context: *mut llvm::LLVMContext) -> IRPointer<LLVMTypeRef> {
    let raw_ptr = unsafe {
        core::LLVMVoidTypeInContext(context)
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// integer type
pub fn int_type(context: *mut llvm::LLVMContext) -> IRPointer<LLVMTypeRef> {
    let raw_ptr = unsafe {
        core::LLVMIntTypeInContext(context, 64)
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// float type
pub fn float_type(context: *mut llvm::LLVMContext) -> IRPointer<LLVMTypeRef> {
    let raw_ptr = unsafe {
        core::LLVMFloatTypeInContext(context)
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// boolean type
pub fn boolean_type(context: *mut llvm::LLVMContext) -> IRPointer<LLVMTypeRef> {
    let raw_ptr = unsafe {
        core::LLVMInt1TypeInContext(context)
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// pointer type
pub fn pointer_type(element_type: LLVMTypeRef) -> IRPointer<LLVMTypeRef> {
    let raw_ptr = unsafe {
        core::LLVMPointerType(element_type, 0)
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// array type
pub fn array_type(element_type: LLVMTypeRef, num_elements: u64) -> IRPointer<LLVMTypeRef> {
    let raw_ptr = unsafe {
        core::LLVMArrayType2(element_type, num_elements)
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// struct type
pub fn struct_type(context: *mut llvm::LLVMContext, element_types: &[LLVMTypeRef], packed: bool) -> IRPointer<LLVMTypeRef> {
    let raw_ptr = unsafe {
        core::LLVMStructTypeInContext(context, element_types.as_ptr() as *mut _, element_types.len() as u32, packed as i32)
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// returns nothing
pub fn void_return(builder: *mut llvm::LLVMBuilder) -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        core::LLVMBuildRetVoid(builder)
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}

/// returns something
pub fn nonvoid_return(builder: *mut llvm::LLVMBuilder, value: LLVMValueRef) -> IRPointer<LLVMValueRef> {
    let raw_ptr = unsafe {
        core::LLVMBuildRet(builder, value)
    };
    IRPointer::new(Some(raw_ptr as *mut _))
}
