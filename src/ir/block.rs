extern crate llvm_sys as llvm;

use llvm::{core, prelude::{LLVMBasicBlockRef, LLVMBuilderRef, LLVMContextRef, LLVMValueRef}, };

use std::ffi::CString;

use crate::memory_management::{ 
    pointer::{LLVMRef, LLVMRefType}, 
    resource_pools::{BasicBlockTag, BuilderTag, ContextTag, ResourcePools, ValueTag}
};

impl ResourcePools {
    /// Creates a basic block in the specified function and context.
    pub fn create_basic_block(
        &mut self,
        context_tag: ContextTag,
        function_tag: ValueTag,
        name: &str
    ) -> Option<BasicBlockTag> {
        let context_arc_rwlock = self.get_context(context_tag)?;
        let function_arc_rwlock = self.get_value(function_tag)?;

        let context_ptr: LLVMContextRef = {
            let context_rwlock = context_arc_rwlock.read().expect("Failed to lock context for reading");
            let context_ref = context_rwlock.read(LLVMRefType::Context, |context_ref| {
                match context_ref {
                    LLVMRef::Context(ptr) => Some(*ptr),
                    _ => None
                }
            })?;
            context_ref
        };
    
        let function_ptr: LLVMValueRef = {
            let function_rwlock = function_arc_rwlock.read().expect("Failed to lock function for reading");
            let function_ref = function_rwlock.read(LLVMRefType::Value, |function_ref| {
                match function_ref {
                    LLVMRef::Value(ptr) => Some(*ptr),
                    _ => None
                }
            })?;
            function_ref
        };
    
        // Create the basic block
        let c_name = CString::new(name).expect("Failed to create CString");
        let basic_block = unsafe {
            core::LLVMAppendBasicBlockInContext(context_ptr, function_ptr, c_name.as_ptr())
        };
    
        if !basic_block.is_null() {
            self.store_basic_block(basic_block)
        } else {
            None
        }
    }

    /// Inserts a basic block before the specified target block in the given context.
    pub fn insert_before_basic_block(&mut self, context_tag: ContextTag, before_target_tag: BasicBlockTag, name: &str) -> Option<BasicBlockTag> {
        let context_arc_rwlock = self.get_context(context_tag)?;
        let before_target_arc_rwlock = self.get_basic_block(before_target_tag)?;

        let context_ptr: LLVMContextRef = {
            let context_rwlock = context_arc_rwlock.read().expect("Failed to lock context for reading");
            let context_ref = context_rwlock.read(LLVMRefType::Context, |context_ref| {
                match context_ref {
                    LLVMRef::Context(ptr) => Some(*ptr),
                    _ => None
                }
            })?;
            context_ref
        };

        let before_target_ptr: LLVMBasicBlockRef = {
            let before_target_rwlock = before_target_arc_rwlock.read().expect("Failed to lock basic block for reading");
            let before_target_ref = before_target_rwlock.read(LLVMRefType::BasicBlock, |bb_ref| {
                match bb_ref {
                    LLVMRef::BasicBlock(ptr) => Some(*ptr),
                    _ => None
                }
            })?;
            before_target_ref
        };

        let c_name = CString::new(name).expect("Failed to create CString");

        let basic_block = unsafe {
            core::LLVMInsertBasicBlockInContext(context_ptr, before_target_ptr, c_name.as_ptr())
        };

        if basic_block.is_null() {
            None
        } else {
            self.store_basic_block(basic_block)
        }
    }

    /// Retrieves the current insertion block. 
    pub fn get_current_block(&mut self, builder_tag: BuilderTag) -> Option<BasicBlockTag> {
        let builder_arc_rwlock = self.get_builder(builder_tag)?;

        let builder_ptr: LLVMBuilderRef = {
            let builder_rwlock = builder_arc_rwlock.read().expect("Failed to lock builder for reading");
            let builder_ref = builder_rwlock.read(LLVMRefType::Builder, |builder_ref| {
                match builder_ref {
                    LLVMRef::Builder(ptr) => Some(*ptr),
                    _ => None
                }
            })?;
            builder_ref
        };

        let block = unsafe {
            core::LLVMGetInsertBlock(builder_ptr)
        };

        if block.is_null() {
            None
        } else {
            self.get_basic_block_tag(block)
        }
    }
    
    /// Creates a conditional branch to two different blocks. 
    pub fn create_cond_br(&mut self, builder_tag: BuilderTag, condition_tag: ValueTag, then_bb_tag: BasicBlockTag, else_bb_tag: BasicBlockTag) -> Option<ValueTag> {
        let builder_arc_rwlock = self.get_builder(builder_tag)?;
        let condition_arc_rwlock = self.get_value(condition_tag)?;
        let then_bb_arc_rwlock = self.get_basic_block(then_bb_tag)?;
        let else_bb_arc_rwlock = self.get_basic_block(else_bb_tag)?;

        let builder_ptr: LLVMBuilderRef = {
            let builder_rwlock = builder_arc_rwlock.read().expect("Failed to lock builder for reading");
            builder_rwlock.read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?
        };

        let condition_ptr: LLVMValueRef = {
            let condition_rwlock = condition_arc_rwlock.read().expect("Failed to lock condition for reading");
            condition_rwlock.read(LLVMRefType::Value, |value_ref| {
                if let LLVMRef::Value(ptr) = value_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?
        };

        let then_bb_ptr: LLVMBasicBlockRef = {
            let then_bb_rwlock = then_bb_arc_rwlock.read().expect("Failed to lock then_bb for reading");
            then_bb_rwlock.read(LLVMRefType::BasicBlock, |bb_ref| {
                if let LLVMRef::BasicBlock(ptr) = bb_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?
        };

        let else_bb_ptr: LLVMBasicBlockRef = {
            let else_bb_rwlock = else_bb_arc_rwlock.read().expect("Failed to lock else_bb for reading");
            else_bb_rwlock.read(LLVMRefType::BasicBlock, |bb_ref| {
                if let LLVMRef::BasicBlock(ptr) = bb_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?
        };

        let branch = unsafe {
            core::LLVMBuildCondBr(builder_ptr, condition_ptr, then_bb_ptr, else_bb_ptr)
        };

        if branch.is_null() {
            None
        } else {
            self.store_value(branch)
        }
    }

    /// Creates an unconditional branch instruction to a target block. 
    pub fn create_br(&mut self, builder_tag: BuilderTag, target_bb_tag: BasicBlockTag) -> Option<ValueTag> {
        let builder_arc_rwlock = self.get_builder(builder_tag)?;
        let target_bb_arc_rwlock = self.get_basic_block(target_bb_tag)?;

        let builder_ptr: LLVMBuilderRef = {
            let builder_rwlock = builder_arc_rwlock.read().expect("Failed to lock builder for reading");
            builder_rwlock.read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?
        };

        let target_bb_ptr: LLVMBasicBlockRef = {
            let target_bb_rwlock = target_bb_arc_rwlock.read().expect("Failed to lock target_bb for reading");
            target_bb_rwlock.read(LLVMRefType::BasicBlock, |bb_ref| {
                if let LLVMRef::BasicBlock(ptr) = bb_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?
        };

        let branch = unsafe {
            core::LLVMBuildBr(builder_ptr, target_bb_ptr)
        };

        if branch.is_null() {
            None
        } else {
            self.store_value(branch)
        }
    }
    
    /// Positions the builder at the end of a block
    pub fn position_builder(&mut self, builder_tag: BuilderTag, bb_tag: BasicBlockTag) -> Option<()> {
        let builder_arc_rwlock = self.get_builder(builder_tag)?;
        let bb_arc_rwlock = self.get_basic_block(bb_tag)?;

        let builder_ptr: LLVMBuilderRef = {
            let builder_rwlock = builder_arc_rwlock.read().expect("Failed to lock builder for reading");
            let builder_ref = builder_rwlock.read(LLVMRefType::Builder, |builder_ref| {
                match builder_ref {
                    LLVMRef::Builder(ptr) => Some(*ptr),
                    _ => None
                }
            })?;
            builder_ref
        };

        let bb_ptr: LLVMBasicBlockRef = {
            let bb_rwlock = bb_arc_rwlock.read().expect("Failed to lock basic block for reading");
            let bb_ref = bb_rwlock.read(LLVMRefType::BasicBlock, |bb_ref| {
                match bb_ref {
                    LLVMRef::BasicBlock(ptr) => Some(*ptr),
                    _ => None
                }
            })?;
            bb_ref
        };

        unsafe {
            core::LLVMPositionBuilderAtEnd(builder_ptr, bb_ptr);
        }

        Some(())
    }

    /// Deletes a specified block.
    pub fn delete_basic_block(&mut self, bb_tag: BasicBlockTag) -> Option<()> {
        let bb_arc_rwlock = self.get_basic_block(bb_tag)?;

        let bb_ptr: LLVMBasicBlockRef = {
            let bb_rwlock = bb_arc_rwlock.read().expect("Failed to lock basic block for reading");
            let bb_ref = bb_rwlock.read(LLVMRefType::BasicBlock, |bb_ref| {
                match bb_ref {
                    LLVMRef::BasicBlock(ptr) => Some(*ptr),
                    _ => None
                }
            })?;
            bb_ref
        };

        unsafe {
            core::LLVMDeleteBasicBlock(bb_ptr);
        }

        Some(())
    }

    /// Retrieves the first instruction in a target block. 
    pub fn get_first_instruction(&mut self, bb_tag: BasicBlockTag) -> Option<ValueTag> {
        let bb_arc_rwlock = self.get_basic_block(bb_tag)?;

        let bb_ptr: LLVMBasicBlockRef = {
            let bb_rwlock = bb_arc_rwlock.read().expect("Failed to lock basic block for reading");
            let bb_ref = bb_rwlock.read(LLVMRefType::BasicBlock, |bb_ref| {
                match bb_ref {
                    LLVMRef::BasicBlock(ptr) => Some(*ptr),
                    _ => None
                }
            })?;
            bb_ref
        };

        let instruction = unsafe {
            core::LLVMGetFirstInstruction(bb_ptr)
        };

        if instruction.is_null() {
            None
        } else {
            self.store_value(instruction)
        }
    }

    /// Retrieves the last instruction in a target block. 
    pub fn get_last_instruction(&mut self, bb_tag: BasicBlockTag) -> Option<ValueTag> {
        let bb_arc_rwlock = self.get_basic_block(bb_tag)?;

        let bb_ptr: LLVMBasicBlockRef = {
            let bb_rwlock = bb_arc_rwlock.read().expect("Failed to lock basic block for reading");
            let bb_ref = bb_rwlock.read(LLVMRefType::BasicBlock, |bb_ref| {
                match bb_ref {
                    LLVMRef::BasicBlock(ptr) => Some(*ptr),
                    _ => None
                }
            })?;
            bb_ref
        };

        let instruction = unsafe {
            core::LLVMGetLastInstruction(bb_ptr)
        };

        if instruction.is_null() {
            None
        } else {
            self.store_value(instruction)
        }
    }

    // /// Creates a PHI node in the specified basic block
    // pub fn create_phi(&mut self, builder_tag: BuilderTag, possible_values: &[(ValueTag, BasicBlockTag)], name: &str) -> Option<ValueTag> {
    //     let builder_arc_rwlock = self.get_builder(builder_tag)?;
    //     let builder_ptr: LLVMBuilderRef = unsafe {
    //         let builder_rwlock = builder_arc_rwlock.read().expect("Failed to lock builder for reading");
    //         let builder_ref = builder_rwlock.read(LLVMRefType::Builder, |builder_ref| {
    //             if let LLVMRef::Builder(ptr) = builder_ref {
    //                 Some(*ptr)
    //             } else {
    //                 None
    //             }
    //         })?;
    //         builder_ref
    //     };

    //     let first_value_tag = possible_values.get(0).map(|(val_tag, _)| *val_tag).expect("No values provided for PHI node");
    //     let first_value_arc_rwlock = self.get_value(first_value_tag)?;
    //     let phi_type = unsafe {
    //         let first_value_rwlock = first_value_arc_rwlock.read().expect("Failed to lock value for reading");
    //         let first_value_ref = first_value_rwlock.read(LLVMRefType::Value, |value_ref| {
    //             if let LLVMRef::Value(ptr) = value_ref {
    //                 Some(core::LLVMTypeOf(*ptr))
    //             } else {
    //                 None
    //             }
    //         })?;
    //         first_value_ref
    //     };

    //     let c_name = CString::new(name).expect("Failed to create CString");

    //     let phi_node = unsafe {
    //         core::LLVMBuildPhi(builder_ptr, phi_type, c_name.as_ptr())
    //     };

    //     if phi_node.is_null() {
    //         None
    //     } else {
    //         let mut values = Vec::new();
    //         let mut blocks = Vec::new();
    //         for &(val_tag, block_tag) in possible_values {
    //             let value_ptr = self.get_value(val_tag)?.read(LLVMRefType::Value, |value_ref| {
    //                 if let LLVMRef::Value(ptr) = value_ref {
    //                     Some(unsafe { *ptr })
    //                 } else {
    //                     None
    //                 }
    //             })?;

    //             let block_ptr = self.get_basic_block(block_tag)?.read(LLVMRefType::BasicBlock, |block_ref| {
    //                 if let LLVMRef::BasicBlock(ptr) = block_ref {
    //                     Some(unsafe { *ptr })
    //                 } else {
    //                     None
    //                 }
    //             })?;

    //             values.push(value_ptr);
    //             blocks.push(block_ptr);
    //         }

    //         unsafe {
    //             core::LLVMAddIncoming(phi_node, values.as_mut_ptr(), blocks.as_mut_ptr(), values.len() as u32);
    //         }

    //         self.store_value(phi_node)
    //     }
    // }
}
