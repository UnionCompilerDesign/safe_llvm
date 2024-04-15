use llvm::prelude::{LLVMContextRef, LLVMValueRef, LLVMBasicBlockRef, LLVMBuilderRef, LLVMTypeRef, LLVMModuleRef};
use std::ffi::CString;

use crate::{
    interface::LLVMApi,
    ir_codegen::SafeLLVM as IrCodeGenSafeLLVM,
    memory_management::ir_pointer::IRPointer,
}; 

pub struct SafeLLVM {
    ir_codegen_impl: IrCodeGenSafeLLVM,
}

impl LLVMApi for SafeLLVM {
    /// --- BLOCK --- ///
    fn create_basic_block(&self, context: LLVMContextRef, function: LLVMValueRef, name: &str) -> IRPointer<LLVMBasicBlockRef> {
        self.ir_codegen_impl.create_basic_block(context, function, name)
    }

    fn get_current_block(&self, builder: LLVMBuilderRef) -> IRPointer<LLVMBasicBlockRef> {
        self.ir_codegen_impl.get_current_block(builder)
    }

    fn create_cond_br(&self, builder: LLVMBuilderRef, condition: LLVMValueRef, then_bb: LLVMBasicBlockRef, else_bb: LLVMBasicBlockRef) -> IRPointer<LLVMValueRef> {
        self.ir_codegen_impl.create_cond_br(builder, condition, then_bb, else_bb)
    }

    fn create_br(&self, builder: LLVMBuilderRef, target_bb: LLVMBasicBlockRef) -> IRPointer<LLVMValueRef> {
        self.ir_codegen_impl.create_br(builder, target_bb)
    }

    fn insert_before_basic_block(&self, context: LLVMContextRef, before_target: LLVMBasicBlockRef, name: &str) -> IRPointer<LLVMBasicBlockRef> {
        self.ir_codegen_impl.insert_before_basic_block(context, before_target, name)
    }

    fn position_builder(&self, builder: LLVMBuilderRef, bb: LLVMBasicBlockRef) {
        self.ir_codegen_impl.position_builder(builder, bb)
    }

    fn delete_basic_block(&self, bb: LLVMBasicBlockRef) {
        self.ir_codegen_impl.delete_basic_block(bb)
    }

    fn get_first_instruction(&self, bb: LLVMBasicBlockRef) -> IRPointer<LLVMValueRef> {
        self.ir_codegen_impl.get_first_instruction(bb)
    }

    fn get_last_instruction(&self, bb: LLVMBasicBlockRef) -> IRPointer<LLVMValueRef> {
        self.ir_codegen_impl.get_last_instruction(bb)
    }

    fn create_phi(&self, builder: LLVMBuilderRef, possible_values: &[(LLVMValueRef, LLVMBasicBlockRef)], name: &str) -> IRPointer<LLVMValueRef> {
        self.ir_codegen_impl.create_phi(builder, possible_values, name)
    }

    /// --- BUILDER --- ///
    fn create_builder(&self, context: LLVMContextRef) -> IRPointer<LLVMBuilderRef> {
        self.ir_codegen_impl.create_builder(context)
    }


    /// --- OPS --- ///
    fn build_add(&self, builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, sum: CString) {
        self.ir_codegen_impl.build_add(builder, param_a, param_b, sum);
    }

    fn build_sub(&self, builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString) {
        self.ir_codegen_impl.build_sub(builder, param_a, param_b, name);
    }

    fn build_mul(&self, builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString) {
        self.ir_codegen_impl.build_mul(builder, param_a, param_b, name);
    }

    fn build_div(&self, builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString) {
        self.ir_codegen_impl.build_div(builder, param_a, param_b, name);
    }

    fn build_rem(&self, builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString) {
        self.ir_codegen_impl.build_rem(builder, param_a, param_b, name);
    }

    fn build_and(&self, builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString) {
        self.ir_codegen_impl.build_and(builder, param_a, param_b, name);
    }

    fn build_or(&self, builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString) {
        self.ir_codegen_impl.build_or(builder, param_a, param_b, name);
    }

    fn build_xor(&self, builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString) {
        self.ir_codegen_impl.build_xor(builder, param_a, param_b, name);
    }

    fn build_shl(&self, builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString) {
        self.ir_codegen_impl.build_shl(builder, param_a, param_b, name);
    }

    fn build_shr(&self, builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString) {
        self.ir_codegen_impl.build_shr(builder, param_a, param_b, name);
    }

    fn build_icmp_gt(&self, builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString) {
        self.ir_codegen_impl.build_icmp_gt(builder, param_a, param_b, name);
    }

    fn build_icmp_lt(&self, builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString) {
        self.ir_codegen_impl.build_icmp_lt(builder, param_a, param_b, name);
    }

    fn build_icmp_eq(&self, builder: LLVMBuilderRef, param_a: LLVMValueRef, param_b: LLVMValueRef, name: CString) {
        self.ir_codegen_impl.build_icmp_eq(builder, param_a, param_b, name);
    }

    fn build_negation(&self, builder: LLVMBuilderRef, operand_ir: LLVMValueRef, name: CString) -> IRPointer<LLVMValueRef> {
        self.ir_codegen_impl.build_negation(builder, operand_ir, name)
    }

    fn build_bitwise_not(&self, builder: LLVMBuilderRef, operand_ir: LLVMValueRef, name: CString) -> IRPointer<LLVMValueRef> {
        self.ir_codegen_impl.build_bitwise_not(builder, operand_ir, name)
    }

    fn build_logical_not(&self, builder: LLVMBuilderRef, context: LLVMContextRef, operand_ir: LLVMValueRef, name: CString) -> IRPointer<LLVMValueRef> {
        self.ir_codegen_impl.build_logical_not(builder, context, operand_ir, name)
    }


    /// --- TOP LEVEL EXP --- ///
    fn get_param(&self, function: LLVMValueRef, index: u32) -> IRPointer<LLVMValueRef> {
        self.ir_codegen_impl.get_param(function, index)
    }
    
    fn add_function_to_module(&self, module: LLVMModuleRef, function_name: &str, function_type: LLVMTypeRef) -> IRPointer<LLVMValueRef> {
        self.ir_codegen_impl.add_function_to_module(module, function_name, function_type)
    }

    /// --- TYPES --- ///
    fn void_type(&self, context: LLVMContextRef) -> IRPointer<LLVMTypeRef> {
        self.ir_codegen_impl.void_type(context)
    }

    fn int_type(&self, context: LLVMContextRef) -> IRPointer<LLVMTypeRef> {
        self.ir_codegen_impl.int_type(context)
    }

    fn float_type(&self, context: LLVMContextRef) -> IRPointer<LLVMTypeRef> {
        self.ir_codegen_impl.float_type(context)
    }

    fn boolean_type(&self, context: LLVMContextRef) -> IRPointer<LLVMTypeRef> {
        self.ir_codegen_impl.boolean_type(context)
    }

    fn pointer_type(&self, element_type: LLVMTypeRef) -> IRPointer<LLVMTypeRef> {
        self.ir_codegen_impl.pointer_type(element_type)
    }

    fn array_type(&self, element_type: LLVMTypeRef, num_elements: u64) -> IRPointer<LLVMTypeRef> {
        self.ir_codegen_impl.array_type(element_type, num_elements)
    }

    fn struct_type(&self, context: LLVMContextRef, element_types: &[LLVMTypeRef], packed: bool) -> IRPointer<LLVMTypeRef> {
        self.ir_codegen_impl.struct_type(context, element_types, packed)
    }

    fn void_return(&self, builder: LLVMBuilderRef) -> IRPointer<LLVMValueRef> {
        self.ir_codegen_impl.void_return(builder)
    }

    fn nonvoid_return(&self, builder: LLVMBuilderRef, value: LLVMValueRef) -> IRPointer<LLVMValueRef> {
        self.ir_codegen_impl.nonvoid_return(builder, value)
    }

    /// --- VALUES --- ///
    fn create_integer(&self, val: i64, context: LLVMContextRef) -> IRPointer<LLVMValueRef> {
        self.ir_codegen_impl.create_integer(val, context)
    }

    fn create_float(&self, val: f64, context: LLVMContextRef) -> IRPointer<LLVMValueRef> {
        self.ir_codegen_impl.create_float(val, context)
    }

    fn create_boolean(&self, val: bool, context: LLVMContextRef) -> IRPointer<LLVMValueRef> {
        self.ir_codegen_impl.create_boolean(val, context)
    }

    fn create_function(&self, name: &str, return_type: Option<LLVMTypeRef>, param_types: &[LLVMTypeRef], is_var_arg: bool, module: LLVMModuleRef) -> IRPointer<LLVMValueRef> {
        self.ir_codegen_impl.create_function(name, return_type, param_types, is_var_arg, module)
    }

    fn create_array(&self, value: LLVMValueRef, num_elements: u64) -> IRPointer<LLVMValueRef> {
        self.ir_codegen_impl.create_array(value, num_elements)
    }

    fn create_pointer(&self, value: LLVMValueRef) -> IRPointer<LLVMValueRef> {
        self.ir_codegen_impl.create_pointer(value)
    }

    fn create_struct(&self, values: &[LLVMValueRef], context: LLVMContextRef, packed: bool) -> IRPointer<LLVMValueRef> {
        self.ir_codegen_impl.create_struct(values, context, packed)
    }

    fn create_global_variable(&self, module: LLVMModuleRef, initializer: LLVMValueRef, name: &str) -> IRPointer<LLVMValueRef> {
        self.ir_codegen_impl.create_global_variable(module, initializer, name)
    }

    fn create_string(&self, val: &str, builder: LLVMBuilderRef) -> IRPointer<LLVMValueRef> {
        self.ir_codegen_impl.create_string(val, builder)
    }

    fn create_mut_string(&self, val: &str, context: LLVMContextRef, builder: LLVMBuilderRef) -> IRPointer<LLVMValueRef> {
        self.ir_codegen_impl.create_mut_string(val, context, builder)
    }

    fn create_null_pointer(&self, ty: LLVMTypeRef) -> IRPointer<LLVMValueRef> {
        self.ir_codegen_impl.create_null_pointer(ty)
    }

    fn create_continue_statement(&self, builder: LLVMBuilderRef, continue_block: LLVMBasicBlockRef) {
        self.ir_codegen_impl.create_continue_statement(builder, continue_block)
    }

    fn create_break_statement(&self, builder: LLVMBuilderRef, break_block: LLVMBasicBlockRef) {
        self.ir_codegen_impl.create_break_statement(builder, break_block)
    }

    fn create_function_type(&self, return_type: LLVMTypeRef, param_types: &[LLVMTypeRef], is_var_arg: bool) -> IRPointer<LLVMTypeRef> {
        self.ir_codegen_impl.create_function_type(return_type, param_types, is_var_arg)
    }

    /// --- VARIABLES --- ///
    fn init_var(&self, builder: LLVMBuilderRef, var_name: &str, data_type: LLVMTypeRef, initial_value: Option<LLVMValueRef>) -> IRPointer<LLVMValueRef> {
        self.ir_codegen_impl.init_var(builder, var_name, data_type, initial_value)
    }

    fn reassign_var(&self, builder: LLVMBuilderRef, variable_alloc: LLVMValueRef, new_value: LLVMValueRef) {
        self.ir_codegen_impl.reassign_var(builder, variable_alloc, new_value)
    }

    fn get_var(&self, builder: LLVMBuilderRef, variable_type: LLVMTypeRef, variable_alloc: LLVMValueRef) -> IRPointer<LLVMValueRef> {
        self.ir_codegen_impl.get_var(builder, variable_type, variable_alloc)
    }
}
