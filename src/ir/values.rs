extern crate llvm_sys as llvm;

use llvm::{core, prelude::LLVMValueRef};

use std::ffi::CString;

use crate::memory_management::{pointer::{LLVMRef, LLVMRefType}, resource_pools::{BuilderHandle, ContextHandle, ModuleHandle, ResourcePools, TypeHandle, ValueHandle}};

impl ResourcePools {
    /// Creates an integer constant of 64 bits in the specified context.
    pub fn create_integer(&mut self, context_handle: ContextHandle, val: i64) -> Option<ValueHandle> {
        let context_arc_rwlock = self.get_context(context_handle)?;
        let integer_value = {
            let context_rwlock = context_arc_rwlock.read().expect("Failed to lock context for reading");
            let context_ptr = context_rwlock.read(LLVMRefType::Context, |context_ref| {
                if let LLVMRef::Context(ptr) = context_ref {
                    Some(unsafe {
                        core::LLVMConstInt(core::LLVMInt64TypeInContext(*ptr), val as u64, 0)
                    })
                } else {
                    None
                }
            })?;
            context_ptr
        };

        if integer_value.is_null() {
            None
        } else {
            self.store_value(integer_value)
        }
    }

    /// Creates a floating-point constant in the specified context.
    pub fn create_float(&mut self, context_handle: ContextHandle, val: f64) -> Option<ValueHandle> {
        let context_arc_rwlock = self.get_context(context_handle)?;
        let float_value = {
            let context_rwlock = context_arc_rwlock.read().expect("Failed to lock context for reading");
            let context_ptr = context_rwlock.read(LLVMRefType::Context, |context_ref| {
                if let LLVMRef::Context(ptr) = context_ref {
                    Some(unsafe { core::LLVMConstReal(core::LLVMDoubleTypeInContext(*ptr), val) })
                } else {
                    None
                }
            })?;
            context_ptr
        };

        if float_value.is_null() {
            None
        } else {
            self.store_value(float_value)
        }
    }

    /// Creates a boolean
    pub fn create_boolean(&mut self, context_handle: ContextHandle, val: bool) -> Option<ValueHandle> {
        let context_arc_rwlock = self.get_context(context_handle)?;
        let boolean_value = {
            let context_rwlock = context_arc_rwlock.read().expect("Failed to lock context for reading");
            let context_ptr = context_rwlock.read(LLVMRefType::Context, |context_ref| {
                if let LLVMRef::Context(ptr) = context_ref {
                    Some(unsafe { core::LLVMConstInt(core::LLVMInt1TypeInContext(*ptr), val as u64, 0) })
                } else {
                    None
                }
            })?;
            context_ptr
        };

        if boolean_value.is_null() {
            None
        } else {
            self.store_value(boolean_value)
        }
    }

    /// Creates an array
    pub fn create_array(&mut self, value_handle: ValueHandle, num_elements: u64) -> Option<ValueHandle> {
        let value_arc_rwlock = self.get_value(value_handle)?;
        
        let array_type = unsafe {
            let value_ptr = value_arc_rwlock.read().expect("Failed to lock value for reading").read(LLVMRefType::Value, |value_ref| {
                if let LLVMRef::Value(ptr) = value_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let element_type = core::LLVMTypeOf(value_ptr);
            let mut values = vec![value_ptr; num_elements as usize];
            core::LLVMConstArray2(element_type, values.as_mut_ptr(), values.len() as u64)
        };

        if array_type.is_null() {
            None
        } else {
            self.store_value(array_type)
        }
    }

    /// Creates a pointer
    pub fn create_pointer(&mut self, element_type_handle: TypeHandle) -> Option<ValueHandle> {
        let element_type_arc_rwlock = self.get_type(element_type_handle)?;

        let pointer_type = unsafe {
            let element_type_ptr = element_type_arc_rwlock.read().expect("Failed to lock type for reading").read(LLVMRefType::Type, |element_type_ref| {
                if let LLVMRef::Type(ptr) = element_type_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            core::LLVMConstPointerNull(core::LLVMPointerType(element_type_ptr, 0))
        };

        if pointer_type.is_null() {
            None
        } else {
            self.store_value(pointer_type)
        }
    }

    /// Creates a struct
    pub fn create_struct(
        &mut self,
        values: &[ValueHandle],
        context_handle: ContextHandle,
        packed: bool
    ) -> Option<ValueHandle> {
        let context_arc_rwlock = self.get_context(context_handle)?;
        let mut value_ptrs: Vec<LLVMValueRef> = values.iter().map(|&handle| {
            self.get_value(handle).and_then(|value_arc_rwlock| {
                value_arc_rwlock.read().expect("Failed to lock value for reading").read(LLVMRefType::Value, |value_ref| {
                if let LLVMRef::Value(ptr) = value_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })
            }).expect("Failed to retrieve value pointer")
        }).collect();

        let struct_type = unsafe {
            let context_ptr = context_arc_rwlock.read().expect("Failed to lock context for reading").read(LLVMRefType::Context, |context_ref| {
                if let LLVMRef::Context(ptr) = context_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;
    
            core::LLVMConstStructInContext(context_ptr, value_ptrs.as_mut_ptr(), value_ptrs.len() as u32, packed as i32)
        };
    
        if struct_type.is_null() {
            None
        } else {
            self.store_value(struct_type)
        }
    }
    
    /// Creates a global variable
    pub fn create_global_variable(
        &mut self,
        module_handle: ModuleHandle,
        initializer_handle: ValueHandle,
        name: &str
    ) -> Option<ValueHandle> {
        let module_arc_rwlock = self.get_module(module_handle)?;
        let initializer_arc_rwlock = self.get_value(initializer_handle)?;

        let c_name = CString::new(name).expect("Failed to create CString for global variable name");

        let global_var = unsafe {
            let module_ptr = module_arc_rwlock.read().expect("Failed to lock module for reading").read(LLVMRefType::Module, |module_ref| {
                if let LLVMRef::Module(ptr) = module_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let initializer_ptr = initializer_arc_rwlock.read().expect("Failed to lock initializer for reading").read(LLVMRefType::Value, |initializer_ref| {
                if let LLVMRef::Value(ptr) = initializer_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let global_var = core::LLVMAddGlobal(module_ptr, core::LLVMTypeOf(initializer_ptr), c_name.as_ptr());
            core::LLVMSetInitializer(global_var, initializer_ptr);
            global_var
        };

        if global_var.is_null() {
            None
        } else {
            self.store_value(global_var)
        }
    }

    /// Creates an immutable (global) string
    pub fn create_string(
        &mut self,
        val: &str,
        builder_handle: BuilderHandle
    ) -> Option<ValueHandle> {
        let builder_arc_rwlock = self.get_builder(builder_handle)?;

        let c_val = CString::new(val).expect("Failed to create CString for string value");
        let c_str_name = CString::new("const_str").expect("Failed to create CString for string name");

        let builder_ptr_option = builder_arc_rwlock.read()
            .expect("Failed to lock builder for reading")
            .read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr) 
                } else {
                    None 
                }
            });

        if let Some(builder_ptr) = builder_ptr_option {
            let str_pointer = unsafe {
                core::LLVMBuildGlobalStringPtr(builder_ptr, c_val.as_ptr(), c_str_name.as_ptr())
            };

            if !str_pointer.is_null() {
                return self.store_value(str_pointer);
            }
        }
        
        None
    }

    /// Creates a mutable (local) string
    pub fn create_mut_string(
        &mut self,
        val: &str,
        context_handle: ContextHandle,
        builder_handle: BuilderHandle
    ) -> Option<ValueHandle> {
        let context_arc_rwlock = self.get_context(context_handle)?;
        let builder_arc_rwlock = self.get_builder(builder_handle)?;

        let c_str_name = CString::new("local_str").expect("Failed to create CString for string name");

        let local_str = unsafe {
            let context_ptr = context_arc_rwlock.read().expect("Failed to lock context for reading").read(LLVMRefType::Context, |context_ref| {
                if let LLVMRef::Context(ptr) = context_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let builder_ptr = builder_arc_rwlock.read().expect("Failed to lock builder for reading").read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let i8_type = core::LLVMInt8TypeInContext(context_ptr);
            let str_type = core::LLVMArrayType2(i8_type, val.len() as u64);
            let local_str = core::LLVMBuildAlloca(builder_ptr.clone(), str_type, c_str_name.as_ptr());

            for (i, &byte) in val.as_bytes().iter().enumerate() {
                let index = core::LLVMConstInt(core::LLVMInt32TypeInContext(context_ptr), i as u64, 0);
                let mut indices = [index];
                let gep = core::LLVMBuildGEP2(builder_ptr.clone(), str_type, local_str, indices.as_mut_ptr(), indices.len() as u32, c_str_name.as_ptr());
                core::LLVMBuildStore(builder_ptr.clone(), core::LLVMConstInt(i8_type, byte as u64, 0), gep);
            }

            local_str
        };

        if local_str.is_null() {
            None
        } else {
            self.store_value(local_str)
        }
    }

    /// Creates a null pointer
    pub fn create_null_pointer(&mut self, ty_handle: TypeHandle) -> Option<ValueHandle> {
        let ty_arc_rwlock = self.get_type(ty_handle)?;

        let null_pointer = unsafe {
            let ty_ptr = ty_arc_rwlock.read().expect("Failed to lock type for reading").read(LLVMRefType::Type, |ty_ref| {
                if let LLVMRef::Type(ptr) = ty_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            core::LLVMConstPointerNull(ty_ptr)
        };

        if null_pointer.is_null() {
            None
        } else {
            self.store_value(null_pointer)
        }
    }

    /// Creates a continue statement
    pub fn create_continue_statement(
        &mut self,
        builder_handle: BuilderHandle,
        continue_block_handle: ValueHandle
    ) -> Option<ValueHandle> {
        let builder_arc_rwlock = self.get_builder(builder_handle)?;
        let continue_block_arc_rwlock = self.get_value(continue_block_handle)?;

        let continue_statement = unsafe {
            let builder_ptr = builder_arc_rwlock.read().expect("Failed to lock builder for reading").read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let continue_block_ptr = continue_block_arc_rwlock.read().expect("Failed to lock continue block for reading").read(LLVMRefType::Value, |continue_block_ref| {
                if let LLVMRef::BasicBlock(ptr) = continue_block_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            Some(core::LLVMBuildBr(builder_ptr, continue_block_ptr))
        };

        if let Some(continue_statement) = continue_statement {
            self.store_value(continue_statement)
        } else {
            None
        }
    }

    /// Creates a break statement
    pub fn create_break_statement(
        &mut self,
        builder_handle: BuilderHandle,
        break_block_handle: ValueHandle
    ) -> Option<ValueHandle> {
        let builder_arc_rwlock = self.get_builder(builder_handle)?;
        let break_block_arc_rwlock = self.get_value(break_block_handle)?;

        let break_statement = unsafe {
            let builder_ptr = builder_arc_rwlock.read().expect("Failed to lock builder for reading").read(LLVMRefType::Builder, |builder_ref| {
                if let LLVMRef::Builder(ptr) = builder_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            let break_block_ptr = break_block_arc_rwlock.read().expect("Failed to lock break block for reading").read(LLVMRefType::Value, |break_block_ref| {
                if let LLVMRef::BasicBlock(ptr) = break_block_ref {
                    Some(*ptr)
                } else {
                    None
                }
            })?;

            Some(core::LLVMBuildBr(builder_ptr, break_block_ptr))
        };

        if let Some(break_statement) = break_statement {
            self.store_value(break_statement)
        } else {
            None
        }
    }
}