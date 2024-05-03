extern crate llvm_sys as llvm;

use crate::memory_management::resource_pools::{BasicBlockTag, BuilderTag, ContextTag, ModuleTag, TypeTag, ValueTag};

pub trait LLVMApi {
    /// --- IR CODE GENERATION SECTION --- ///
    /// --- BLOCK --- ///
    fn create_basic_block(&mut self, context: ContextTag, function: ValueTag, name: &str) -> Option<BasicBlockTag>;
    fn get_current_block(&mut self, builder: BuilderTag) -> Option<BasicBlockTag>;
    fn create_cond_br(&mut self, builder: BuilderTag, condition: ValueTag, then_bb: BasicBlockTag, else_bb: BasicBlockTag) -> Option<ValueTag>;
    fn create_br(&mut self, builder: BuilderTag, target_bb: BasicBlockTag) -> Option<ValueTag>;
    fn insert_before_basic_block(&mut self, context: ContextTag, before_target: BasicBlockTag, name: &str) -> Option<BasicBlockTag>;
    fn position_builder(&mut self, builder: BuilderTag, bb: BasicBlockTag) -> Option<()>;
    fn delete_basic_block(&mut self, bb: BasicBlockTag) -> Option<()>;
    fn get_first_instruction(&mut self, bb: BasicBlockTag) -> Option<ValueTag>;
    fn get_last_instruction(&mut self, bb: BasicBlockTag) -> Option<ValueTag>;   
    // fn create_phi(&mut self, builder: BuilderTag, possible_values: &[(ValueTag, BasicBlockTag)], name: &str) -> Option<ValueTag>; 
    
    /// --- BUILDER --- ///
    fn create_builder(&mut self, context: ContextTag) -> Option<BuilderTag>;

    /// --- OPERATIONS --- ///
    fn build_add(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, sum: &str) -> Option<ValueTag>;
    fn build_sub(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag>;
    fn build_mul(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag>;
    fn build_div(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag>;
    fn build_rem(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag>;
    fn build_and(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag>;
    fn build_or(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag>;
    fn build_xor(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag>;
    fn build_shl(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag>;
    fn build_shr(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag>;
    fn build_icmp_gt(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag>;
    fn build_icmp_lt(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag>;
    fn build_icmp_eq(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag>;
    fn build_negation(&mut self, builder: BuilderTag, operand_ir: ValueTag, name: &str) -> Option<ValueTag>;
    fn build_bitwise_not(&mut self, builder: BuilderTag, operand_ir: ValueTag, name: &str) -> Option<ValueTag>;
    fn build_logical_not(&mut self, builder: BuilderTag, context: ContextTag, operand_ir: ValueTag, name: &str) -> Option<ValueTag>;

    /// --- TOP LEVEL EXPRESSIONS --- ///
    fn get_param(&mut self, function: ValueTag, index: u32) -> Option<ValueTag>;
    fn add_function_to_module(&mut self, module: ModuleTag, function_name: &str, function_type: TypeTag) -> Option<ValueTag>;

    /// --- TYPES --- ///    
    fn void_type(&mut self, context: ContextTag) -> Option<TypeTag>;
    fn int_type(&mut self, context: ContextTag, num_bits: u32) -> Option<TypeTag>;
    fn float_type(&mut self, context: ContextTag) -> Option<TypeTag>;
    fn boolean_type(&mut self, context: ContextTag) -> Option<TypeTag>;
    fn pointer_type(&mut self, element_type: TypeTag) -> Option<TypeTag>;
    fn array_type(&mut self, element_type: TypeTag, num_elements: u64) -> Option<TypeTag>;
    fn struct_type(&mut self, context: ContextTag, element_types: &[TypeTag], packed: bool) -> Option<TypeTag>;
    fn void_return(&mut self, builder: BuilderTag) -> Option<ValueTag>;
    fn nonvoid_return(&mut self, builder: BuilderTag, value: ValueTag) -> Option<ValueTag>;

    /// --- VALUES --- ///
    fn create_integer(&mut self, val: i64, context: ContextTag) -> Option<ValueTag>;
    fn create_float(&mut self, val: f64, context: ContextTag) -> Option<ValueTag>;
    fn create_boolean(&mut self, val: bool, context: ContextTag) -> Option<ValueTag>;
    fn create_function(&mut self, return_type: Option<TypeTag>, param_types: &[TypeTag], is_var_arg: bool, context: ContextTag) -> Option<TypeTag>;
    fn create_array(&mut self, value: ValueTag, num_elements: u64) -> Option<ValueTag>;
    fn create_pointer(&mut self, value: TypeTag) -> Option<ValueTag>;
    fn create_string(&mut self, val: &str) -> Option<ValueTag>;
    fn create_mut_string(&mut self, val: &str, context: ContextTag, builder: BuilderTag) -> Option<ValueTag>;
    fn create_null_pointer(&mut self, ty: TypeTag) -> Option<ValueTag>;
    fn create_continue_statement(&mut self, builder: BuilderTag, continue_block: BasicBlockTag) -> Option<ValueTag>;
    fn create_break_statement(&mut self, builder: BuilderTag, break_block: BasicBlockTag) -> Option<ValueTag>;

    /// --- VARIABLES --- ///
    fn init_var(&mut self, builder: BuilderTag, var_name: &str, data_type: TypeTag, initial_value: Option<ValueTag>) -> Option<ValueTag>;
    fn reassign_var(&mut self, builder: BuilderTag, variable_alloc: ValueTag, new_value: ValueTag) -> Option<()>;
    fn get_var(&mut self, builder: BuilderTag, variable_type: TypeTag, variable_alloc: ValueTag) -> Option<ValueTag>;
}
