extern crate llvm_sys as llvm;

use std::ffi::CString;

use llvm::prelude::{LLVMBasicBlockRef, LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef};

use crate::memory_management::ir_pointer::IRPointer;

pub trait LLVMApi {
    /// --- IR CODE GENERATION SECTION --- ///
    /// --- BLOCK --- ///
    fn create_basic_block(&self, context: LLVMContextRef, function: LLVMValueRef, name: &str) -> IRPointer<LLVMBasicBlockRef>;
    fn get_current_block(&self, builder: LLVMBuilderRef) -> IRPointer<LLVMBasicBlockRef>;
    fn create_cond_br(&self, builder: LLVMBuilderRef, condition: LLVMValueRef, then_bb: LLVMBasicBlockRef, else_bb: LLVMBasicBlockRef) -> IRPointer<LLVMValueRef>;
    fn create_br(&self, builder: LLVMBuilderRef, target_bb: LLVMBasicBlockRef) -> IRPointer<LLVMValueRef>;
    fn insert_before_basic_block(&self, context: LLVMContextRef, before_target: LLVMBasicBlockRef, name: &str) -> IRPointer<LLVMBasicBlockRef>;
    fn position_builder(&self, builder: LLVMBuilderRef, bb: LLVMBasicBlockRef);
    fn delete_basic_block(&self, bb: LLVMBasicBlockRef);
    fn get_first_instruction(&self, bb: LLVMBasicBlockRef) -> IRPointer<LLVMValueRef>;
    fn get_last_instruction(&self, bb: LLVMBasicBlockRef) -> IRPointer<LLVMValueRef>;   
    fn create_phi(&self, builder: LLVMBuilderRef, possible_values: &[(LLVMValueRef, LLVMBasicBlockRef)], name: &str) -> IRPointer<LLVMValueRef>; 
    
    /// --- BUILDER --- ///
    fn create_builder(context: LLVMContextRef) -> LLVMBuilderRef;

    /// --- OPERATIONS --- ///
    fn build_add(builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, sum: CString);
    fn build_sub(builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString);
    fn build_mul(builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString);
    fn build_div(builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString);
    fn build_rem(builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString);
    fn build_and(builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString);
    fn build_or(builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString);
    fn build_xor(builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString);
    fn build_shl(builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString);
    fn build_shr(builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString);
    fn build_icmp_gt(builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString);
    fn build_icmp_lt(builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString);
    fn build_icmp_eq(builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString);
    fn build_negation(builder: LLVMBuilderRef, operand_ir: LLVMValueRef, name: CString) -> LLVMValueRef;
    fn build_bitwise_not(builder: LLVMBuilderRef, operand_ir: LLVMValueRef, name: CString) -> LLVMValueRef;
    fn build_logical_not(builder: LLVMBuilderRef, context: LLVMContextRef, operand_ir: LLVMValueRef, name: CString) -> LLVMValueRef;

    /// --- TOP LEVEL EXPRESSIONS --- ///
    fn get_param(function: LLVMValueRef, index: u32) -> LLVMValueRef;
    fn add_function_to_module(module: LLVMModuleRef, function_name: &str, function_type: LLVMTypeRef) -> LLVMValueRef;

    /// --- TYPES --- ///    
    fn void_type(context: LLVMContextRef) -> LLVMTypeRef;
    fn int_type(context: LLVMContextRef) -> LLVMTypeRef;
    fn float_type(context: LLVMContextRef) -> LLVMTypeRef;
    fn boolean_type(context: LLVMContextRef) -> LLVMTypeRef;
    fn pointer_type(element_type: LLVMTypeRef) -> LLVMTypeRef;
    fn array_type(element_type: LLVMTypeRef, num_elements: u64) -> LLVMTypeRef;
    fn struct_type(context: LLVMContextRef, element_types: &[LLVMTypeRef], packed: bool) -> LLVMTypeRef;
    fn void_return(builder: LLVMBuilderRef) -> LLVMValueRef;
    fn nonvoid_return(builder: LLVMBuilderRef, value: LLVMValueRef) -> LLVMValueRef;
    
    /// --- VALUES --- ///
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
    fn create_break_statement(builder: LLVMBuilderRef, break_block: LLVMBasicBlockRef);
    fn create_function_type(return_type: LLVMTypeRef, param_types: &[LLVMTypeRef], is_var_arg: bool) -> LLVMTypeRef;

    /// --- VARIABLES --- ///

}

pub struct SafeLLVM;