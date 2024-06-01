//! This module provides functionality for creating and managing basic blocks for generating LLVM's Intermediate Representation (IR).
//!
//! Basic blocks are the core components of functions in LLVM IR, consisting of a sequence of instructions that execute sequentially. 
//! This module allows manipulation of these basic blocks, including creation, insertion, and query operations within a given LLVM context and function.

extern crate llvm_sys as llvm;

use llvm::{core, prelude::{LLVMBasicBlockRef, LLVMBuilderRef, LLVMContextRef, LLVMValueRef}};
use std::ffi::CString;
use common::pointer::{LLVMRef, LLVMRefType};
use crate::core::{BasicBlockTag, BuilderTag, ContextTag, IRGenerator, ValueTag};

impl IRGenerator {
    /// Creates a basic block in the specified function and context.
    ///
    /// # Parameters
    /// * `context_tag` - Context within which the basic block is created.
    /// * `function_tag` - Function to which the basic block will be attached.
    /// * `name` - Name for the new basic block.
    ///
    /// # Returns
    /// Option<BasicBlockTag> - The tag of the newly created basic block, or None if creation fails.
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

    /// Inserts a basic block before a specified target block in a given context.
    ///
    /// # Parameters
    /// * `context_tag` - Context within which the operation is performed.
    /// * `before_target_tag` - BasicBlockTag before which the new block will be inserted.
    /// * `name` - Name for the new basic block.
    ///
    /// # Returns
    /// Option<BasicBlockTag> - The tag of the inserted basic block, or None if insertion fails.
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

    /// Retrieves the currently active insertion block based on the builder tag.
    ///
    /// # Parameters
    /// * `builder_tag` - BuilderTag used to identify the current builder state.
    ///
    /// # Returns
    /// Option<BasicBlockTag> - The tag of the current basic block, or None if no block is active.
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

    /// Retrieves the next basic block following the current one in sequence.
    ///
    /// # Parameters
    /// * `builder_tag` - BuilderTag used to identify the current builder state.
    ///
    /// # Returns
    /// Option<BasicBlockTag> - The tag of the next basic block, or None if there is no subsequent block.
    pub fn get_next_block(&mut self, builder_tag: BuilderTag) -> Option<BasicBlockTag> {
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
            let return_block = unsafe {
                core::LLVMGetNextBasicBlock(block)
            };

            self.get_basic_block_tag(return_block)
        }

    }

    /// Retrieves the previous basic block relative to the current one.
    ///
    /// # Parameters
    /// * `builder_tag` - BuilderTag used to navigate the block structure.
    ///
    /// # Returns
    /// Option<BasicBlockTag> - The tag of the previous basic block, or None if there is no preceding block.
    pub fn get_previous_block(&mut self, builder_tag: BuilderTag) -> Option<BasicBlockTag> {
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
            let return_block = unsafe {
                core::LLVMGetPreviousBasicBlock(block)
            };

            self.get_basic_block_tag(return_block)
        }

    }
    
    /// Creates a conditional branch to two different blocks.
    ///
    /// # Parameters
    /// * `builder_tag` - BuilderTag indicating the current position.
    /// * `condition_tag` - ValueTag representing the condition for branching.
    /// * `then_bb_tag` - BasicBlockTag for the 'then' branch.
    /// * `else_bb_tag` - BasicBlockTag for the 'else' branch.
    ///
    /// # Returns
    /// Option<ValueTag> - The resulting branch instruction tag, or None if the branch creation fails.
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
    ///
    /// # Parameters
    /// * `builder_tag` - BuilderTag indicating the current position.
    /// * `target_bb_tag` - BasicBlockTag of the target block for the branch.
    ///
    /// # Returns
    /// Option<ValueTag> - The tag of the created branch instruction, or None if the operation fails.
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
    
    /// Positions the builder at the end of a specified block for further instruction insertion.
    ///
    /// # Parameters
    /// * `builder_tag` - BuilderTag used for position management.
    /// * `bb_tag` - BasicBlockTag where the builder will be positioned.
    ///
    /// # Returns
    /// Option<()> - None if the operation fails, or an empty Option if successful.
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

    /// Deletes a specified basic block from the function.
    ///
    /// # Parameters
    /// * `bb_tag` - BasicBlockTag of the block to be deleted.
    ///
    /// # Returns
    /// Option<()> - None if the deletion fails, or an empty Option if successful.
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

    /// Retrieves the first instruction within a target basic block.
    ///
    /// # Parameters
    /// * `bb_tag` - BasicBlockTag identifying the block of interest.
    ///
    /// # Returns
    /// Option<ValueTag> - The tag of the first instruction in the block, or None if there are no instructions.
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

    /// Retrieves the last instruction within a target basic block.
    ///
    /// # Parameters
    /// * `bb_tag` - BasicBlockTag identifying the block of interest.
    ///
    /// # Returns
    /// Option<ValueTag> - The tag of the last instruction in the block, or None if there are no instructions.
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
}
