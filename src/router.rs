use crate::{
    interface::LLVMApi,
    ir::SafeLLVM as IrCodeGenSafeLLVM,
    memory_management::resource_pools::{BasicBlockTag, BuilderTag, ContextTag, ModuleTag, TypeTag, ValueTag},
}; 

pub struct SafeLLVM {
    ir_codegen_impl: IrCodeGenSafeLLVM,
}

impl LLVMApi for SafeLLVM {
    /// --- BLOCK --- ///
    fn create_basic_block(&mut self, context: ContextTag, function: ValueTag, name: &str) -> Option<BasicBlockTag> {
        self.ir_codegen_impl.create_basic_block(context, function, name)
    }

    fn create_basic_block_after(&mut self, context_tag: ContextTag, function_tag: ValueTag, target_tag: BasicBlockTag, name: &str) -> Option<BasicBlockTag> {
        self.ir_codegen_impl.create_basic_block_after(context_tag, function_tag, target_tag, name)
    }

    fn get_current_block(&mut self, builder: BuilderTag) -> Option<BasicBlockTag> {
        self.ir_codegen_impl.get_current_block(builder)
    }

    fn create_cond_br(&mut self, builder: BuilderTag, condition: ValueTag, then_bb: BasicBlockTag, else_bb: BasicBlockTag) -> Option<ValueTag> {
        self.ir_codegen_impl.create_cond_br(builder, condition, then_bb, else_bb)
    }

    fn create_br(&mut self, builder: BuilderTag, target_bb: BasicBlockTag) -> Option<ValueTag> {
        self.ir_codegen_impl.create_br(builder, target_bb)
    }

    fn insert_before_basic_block(&mut self, context: ContextTag, before_target: BasicBlockTag, name: &str) -> Option<BasicBlockTag> {
        self.ir_codegen_impl.insert_before_basic_block(context, before_target, name)
    }

    fn position_builder(&mut self, builder: BuilderTag, bb: BasicBlockTag) -> Option<()> {
        self.ir_codegen_impl.position_builder(builder, bb)
    }

    fn delete_basic_block(&mut self, bb: BasicBlockTag) -> Option<()>{
        self.ir_codegen_impl.delete_basic_block(bb)
    }

    fn get_first_instruction(&mut self, bb: BasicBlockTag) -> Option<ValueTag> {
        self.ir_codegen_impl.get_first_instruction(bb)
    }

    fn get_last_instruction(&mut self, bb: BasicBlockTag) -> Option<ValueTag> {
        self.ir_codegen_impl.get_last_instruction(bb)
    }

    // fn create_phi(&mut self, builder: BuilderTag, possible_values: &[(ValueTag, BasicBlockTag)], name: &str) -> Option<ValueTag> {
    //     self.ir_codegen_impl.create_phi(builder, possible_values, name)
    // }

    /// --- BUILDER --- ///
    fn create_builder(&mut self, context: ContextTag) -> Option<BuilderTag> {
        self.ir_codegen_impl.create_builder(context)
    }


    /// --- OPS --- ///
    fn build_add(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, sum: &str) -> Option<ValueTag> {
        self.ir_codegen_impl.build_add(builder, param_a, param_b, sum)
    }

    fn build_sub(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag> {
        self.ir_codegen_impl.build_sub(builder, param_a, param_b, name)
    }

    fn build_mul(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag> {
        self.ir_codegen_impl.build_mul(builder, param_a, param_b, name)
    }

    fn build_div(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag> {
        self.ir_codegen_impl.build_div(builder, param_a, param_b, name)
    }

    fn build_rem(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag> {
        self.ir_codegen_impl.build_rem(builder, param_a, param_b, name)
    }

    fn build_and(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag> {
        self.ir_codegen_impl.build_and(builder, param_a, param_b, name)
    }

    fn build_or(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag> {
        self.ir_codegen_impl.build_or(builder, param_a, param_b, name)
    }

    fn build_xor(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag> {
        self.ir_codegen_impl.build_xor(builder, param_a, param_b, name)
    }

    fn build_shl(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag> {
        self.ir_codegen_impl.build_shl(builder, param_a, param_b, name)
    }

    fn build_shr(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag> {
        self.ir_codegen_impl.build_shr(builder, param_a, param_b, name)
    }

    fn build_icmp_gt(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag> {
        self.ir_codegen_impl.build_icmp_gt(builder, param_a, param_b, name)
    }

    fn build_icmp_lt(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag> {
        self.ir_codegen_impl.build_icmp_lt(builder, param_a, param_b, name)
    }

    fn build_icmp_eq(&mut self, builder: BuilderTag, param_a: ValueTag, param_b: ValueTag, name: &str) -> Option<ValueTag> {
        self.ir_codegen_impl.build_icmp_eq(builder, param_a, param_b, name)
    }

    fn build_negation(&mut self, builder: BuilderTag, operand_ir: ValueTag, name: &str) -> Option<ValueTag> {
        self.ir_codegen_impl.build_negation(builder, operand_ir, name)
    }

    fn build_bitwise_not(&mut self, builder: BuilderTag, operand_ir: ValueTag, name: &str) -> Option<ValueTag> {
        self.ir_codegen_impl.build_bitwise_not(builder, operand_ir, name)
    }

    fn build_logical_not(&mut self, builder: BuilderTag, context: ContextTag, operand_ir: ValueTag, name: &str) -> Option<ValueTag> {
        self.ir_codegen_impl.build_logical_not(builder, context, operand_ir, name)
    }

    fn nonvoid_return(&mut self, builder: BuilderTag, value: ValueTag) -> Option<ValueTag> {
        self.ir_codegen_impl.nonvoid_return(builder, value)
    }

    fn void_return(&mut self, builder: BuilderTag) -> Option<ValueTag> {
        self.ir_codegen_impl.void_return(builder)
    }

    /// --- TOP LEVEL EXP --- ///
    fn get_param(&mut self, function: ValueTag, index: u32) -> Option<ValueTag> {
        self.ir_codegen_impl.get_param(function, index)
    }

    fn add_function_to_module(&mut self, module: ModuleTag, function_name: &str, function_type: TypeTag) -> Option<ValueTag> {
        self.ir_codegen_impl.add_function_to_module(module, function_name, function_type)
    }

    /// --- TYPES --- ///
    fn void_type(&mut self, context: ContextTag) -> Option<TypeTag> {
        self.ir_codegen_impl.void_type(context)
    }

    fn int_type(&mut self, context: ContextTag, num_bits: u32) -> Option<TypeTag> {
        self.ir_codegen_impl.int_type(context, num_bits)
    }

    fn float_type(&mut self, context: ContextTag) -> Option<TypeTag> {
        self.ir_codegen_impl.float_type(context)
    }

    fn boolean_type(&mut self, context: ContextTag) -> Option<TypeTag> {
        self.ir_codegen_impl.boolean_type(context)
    }

    fn pointer_type(&mut self, element_type: TypeTag) -> Option<TypeTag> {
        self.ir_codegen_impl.pointer_type(element_type)
    }

    fn array_type(&mut self, element_type: TypeTag, num_elements: u64) -> Option<TypeTag> {
        self.ir_codegen_impl.array_type(element_type, num_elements)
    }

    fn struct_type(&mut self, context: ContextTag, element_types: &[TypeTag], packed: bool) -> Option<TypeTag> {
        self.ir_codegen_impl.struct_type(context, element_types, packed)
    }

    /// --- VALUES --- ///
    fn create_integer(&mut self, val: i64, context: ContextTag) -> Option<ValueTag> {
        self.ir_codegen_impl.create_integer(val, context)
    }

    fn create_float(&mut self, val: f64, context: ContextTag) -> Option<ValueTag> {
        self.ir_codegen_impl.create_float(val, context)
    }

    fn create_boolean(&mut self, val: bool, context: ContextTag) -> Option<ValueTag> {
        self.ir_codegen_impl.create_boolean(val, context)
    }

    fn create_function(&mut self, return_type: Option<TypeTag>, param_types: &[TypeTag], is_var_arg: bool, context: ContextTag) -> Option<TypeTag> {
        self.ir_codegen_impl.create_function(return_type, param_types, is_var_arg, context)
    }

    fn create_array(&mut self, value: ValueTag, num_elements: u64) -> Option<ValueTag> {
        self.ir_codegen_impl.create_array(value, num_elements)
    }

    fn create_pointer(&mut self, value: TypeTag) -> Option<ValueTag> {
        self.ir_codegen_impl.create_pointer(value)
    }

    fn create_string(&mut self, val: &str) -> Option<ValueTag> {
        self.ir_codegen_impl.create_string(val)
    }

    fn create_mut_string(&mut self, val: &str, context: ContextTag, builder: BuilderTag) -> Option<ValueTag> {
        self.ir_codegen_impl.create_mut_string(val, context, builder)
    }

    fn create_null_pointer(&mut self, ty: TypeTag) -> Option<ValueTag> {
        self.ir_codegen_impl.create_null_pointer(ty)
    }



    /// --- VARIABLES --- ///
    fn init_var(&mut self, builder: BuilderTag, var_name: &str, data_type: TypeTag, initial_value: Option<ValueTag>) -> Option<ValueTag> {
        self.ir_codegen_impl.init_var(builder, var_name, data_type, initial_value)
    }

    fn reassign_var(&mut self, builder: BuilderTag, variable_alloc: ValueTag, new_value: ValueTag) -> Option<()> {
        self.ir_codegen_impl.reassign_var(builder, variable_alloc, new_value)
    }

    fn get_var(&mut self, builder: BuilderTag, variable_type: TypeTag, variable_alloc: ValueTag) -> Option<ValueTag> {
        self.ir_codegen_impl.get_var(builder, variable_type, variable_alloc)
    }
}
