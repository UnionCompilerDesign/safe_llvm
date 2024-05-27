use crate::{
    interface::LLVMApi, 
    memory_management::resource_pools::{BasicBlockTag, BuilderTag, ContextTag, ModuleTag, ResourcePools, TypeTag, ValueTag}
};

pub struct SafeLLVM {
    pools: ResourcePools,
}

impl LLVMApi for SafeLLVM {
    /// --- BLOCK --- ///
    fn create_basic_block(&mut self, context: ContextTag, function: ValueTag, name: &str) -> Option<BasicBlockTag> {
        self.pools.create_basic_block(context, function, name)
    }

    fn get_current_block(&mut self, builder: BuilderTag) -> Option<BasicBlockTag> {
        self.pools.get_current_block(builder)
    }

    fn create_cond_br(&mut self, builder: BuilderTag, condition: ValueTag, then_bb: BasicBlockTag, else_bb: BasicBlockTag) -> Option<ValueTag> {
        self.pools.create_cond_br(builder, condition, then_bb, else_bb)
    }

    fn create_br(&mut self, builder: BuilderTag, target_bb: BasicBlockTag) -> Option<ValueTag> {
        self.pools.create_br(builder, target_bb)
    }

    fn insert_before_basic_block(&mut self, context: ContextTag, before_target: BasicBlockTag, name: &str) -> Option<BasicBlockTag> {
        self.pools.insert_before_basic_block(context, before_target, name)
    }

    fn position_builder(&mut self, builder: BuilderTag, bb: BasicBlockTag) -> Option<()> {
        self.pools.position_builder(builder, bb)
    }

    fn delete_basic_block(&mut self, bb: BasicBlockTag) -> Option<()> {
        self.pools.delete_basic_block(bb)
    }

    fn get_first_instruction(&mut self, bb: BasicBlockTag) -> Option<ValueTag> {
        self.pools.get_first_instruction(bb)
    }

    fn get_last_instruction(&mut self, bb: BasicBlockTag) -> Option<ValueTag> {
        self.pools.get_last_instruction(bb)
    }

    // fn create_phi(&mut self, builder: BuilderTag, possible_values: &[(ValueTag, BasicBlockTag)], name: &str) -> Option<ValueTag> {
    //     self.pools.create_phi(builder, possible_values, name)
    // }

    /// --- BUILDER --- ///
    fn create_builder(&mut self, context: ContextTag) -> Option<BuilderTag> {
        self.pools.create_builder(context)
    }

    fn create_function(&mut self, return_type_tag: Option<TypeTag>, param_type_tags: &[TypeTag], is_var_arg: bool, context_tag: ContextTag) -> Option<TypeTag> {
        self.pools.create_function(return_type_tag, param_type_tags, is_var_arg, context_tag)
    }

    /// --- OPS --- ///
    fn build_add(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, sum: &str) -> Option<ValueTag> {
        self.pools.build_add(builder, param_a, param_b, sum)
    }

    fn build_sub(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag> {
        self.pools.build_sub(builder, param_a, param_b, name)
    }

    fn build_mul(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag> {
        self.pools.build_mul(builder, param_a, param_b, name)
    }

    fn build_div(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag> {
        self.pools.build_div(builder, param_a, param_b, name)
    }

    fn build_rem(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag> {
        self.pools.build_rem(builder, param_a, param_b, name)
    }

    fn build_and(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag> {
        self.pools.build_and(builder, param_a, param_b, name)
    }

    fn build_or(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag> {
        self.pools.build_or(builder, param_a, param_b, name)
    }

    fn build_xor(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag> {
        self.pools.build_xor(builder, param_a, param_b, name)
    }

    fn build_shl(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag> {
        self.pools.build_shl(builder, param_a, param_b, name)
    }

    fn build_shr(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag> {
        self.pools.build_shr(builder, param_a, param_b, name)
    }

    fn build_icmp_gt(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag> {
        self.pools.build_icmp_gt(builder, param_a, param_b, name)
    }

    fn build_icmp_lt(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag> {
        self.pools.build_icmp_lt(builder, param_a, param_b, name)
    }

    fn build_icmp_eq(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag> {
        self.pools.build_icmp_eq(builder, param_a, param_b, name)
    }

    fn build_negation(&mut self, builder: BuilderTag, operand_ir: ValueTag, name: &str) -> Option<ValueTag> {
        self.pools.build_negation(builder, operand_ir, name)
    }

    fn build_bitwise_not(&mut self, builder: BuilderTag, operand_ir: ValueTag, name: &str) -> Option<ValueTag> {
        self.pools.build_bitwise_not(builder, operand_ir, name)
    }

    fn build_logical_not(&mut self, builder: BuilderTag, context: ContextTag, operand_ir: ValueTag, name: &str) -> Option<ValueTag> {
        self.pools.build_logical_not(builder, context, operand_ir, name)
    }

    /// --- TOP LEVEL EXP --- ///
    fn get_param(&mut self, function: ValueTag, index: u32) -> Option<ValueTag> {
        self.pools.get_param(function, index)
    }

    fn add_function_to_module(&mut self, module: ModuleTag, function_name: &str, function_type: TypeTag) -> Option<ValueTag> {
        self.pools.add_function_to_module(module, function_name, function_type)
    }

    /// --- TYPES --- ///
    fn void_type(&mut self, context: ContextTag) -> Option<TypeTag> {
        self.pools.void_type(context)
    }

    fn int_type(&mut self, context: ContextTag, num_bits: u32) -> Option<TypeTag> {
        self.pools.int_type(context, num_bits)
    }

    fn float_type(&mut self, context: ContextTag) -> Option<TypeTag> {
        self.pools.float_type(context)
    }

    fn boolean_type(&mut self, context: ContextTag) -> Option<TypeTag> {
        self.pools.boolean_type(context)
    }

    fn pointer_type(&mut self, element_type: TypeTag) -> Option<TypeTag> {
        self.pools.pointer_type(element_type)
    }

    fn array_type(&mut self, element_type: TypeTag, num_elements: u64) -> Option<TypeTag> {
        self.pools.array_type(element_type, num_elements)
    }

    fn struct_type(&mut self, context: ContextTag, element_types: &[TypeTag], packed: bool) -> Option<TypeTag> {
        self.pools.struct_type(context, element_types, packed)
    }

    fn void_return(&mut self, builder: BuilderTag) -> Option<ValueTag> {
        self.pools.void_return(builder)
    }

    fn nonvoid_return(&mut self, builder: BuilderTag, value: ValueTag) -> Option<ValueTag> {
        self.pools.nonvoid_return(builder, value)
    }


    /// --- VALUES --- ///
    fn create_integer(&mut self, val: i64, context: ContextTag) -> Option<ValueTag> {
        self.pools.create_integer(context, val)
    }

    fn create_float(&mut self, val: f64, context: ContextTag) -> Option<ValueTag> {
        self.pools.create_float(context, val)
    }

    fn create_boolean(&mut self, val: bool, context: ContextTag) -> Option<ValueTag> {
        self.pools.create_boolean(context, val)
    }
    fn create_array(&mut self, value: ValueTag, num_elements: u64) -> Option<ValueTag> {
        self.pools.create_array(value, num_elements)
    }

    fn create_pointer(&mut self, value: TypeTag) -> Option<ValueTag> {
        self.pools.create_pointer(value)
    }

    fn create_string(&mut self, val: &str) -> Option<ValueTag> {
        self.pools.create_string(val)
    }

    fn create_mut_string(&mut self, val: &str, context: ContextTag, builder: BuilderTag) -> Option<ValueTag> {
        self.pools.create_mut_string(val, context, builder)
    }

    fn create_null_pointer(&mut self, ty: TypeTag) -> Option<ValueTag> {
        self.pools.create_null_pointer(ty)
    }

    fn create_basic_block_after(&mut self, context_tag: ContextTag, function_tag: ValueTag, target_tag: BasicBlockTag, name: &str) -> Option<BasicBlockTag> {
        self.pools.create_basic_block_after(context_tag, function_tag, target_tag, name)
    }

    /// --- VARIABLES --- ///
    fn init_var(&mut self, builder: BuilderTag, var_name: &str, data_type: TypeTag, initial_value: Option<ValueTag>) -> Option<ValueTag> {
        self.pools.init_var(builder, var_name, data_type, initial_value)
    }

    fn reassign_var(&mut self, builder: BuilderTag, variable_alloc: ValueTag, new_value: ValueTag) -> Option<()> {
        self.pools.reassign_var(builder, variable_alloc, new_value)
    }

    fn get_var(&mut self, builder: BuilderTag, variable_type: TypeTag, variable_alloc: ValueTag) -> Option<ValueTag> {
        self.pools.get_var(builder, variable_type, variable_alloc)
    }
}
