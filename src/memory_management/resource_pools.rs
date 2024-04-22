extern crate llvm_sys as llvm;

use std::{collections::HashMap, sync::{Arc, RwLock}};
use crate::memory_management::pointer::CPointer;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Handle(usize);

pub struct LLVMResourcePools<C, M, V, B, Bu, T> {
    contexts: Option<HashMap<Handle, Arc<RwLock<CPointer<C>>>>>,
    modules: Option<HashMap<Handle, Arc<RwLock<CPointer<M>>>>>,
    values: Option<HashMap<Handle, Arc<RwLock<CPointer<V>>>>>,
    basic_blocks: Option<HashMap<Handle, Arc<RwLock<CPointer<B>>>>>,
    builders: Option<HashMap<Handle, Arc<RwLock<CPointer<Bu>>>>>,
    types: Option<HashMap<Handle, Arc<RwLock<CPointer<T>>>>>,
    next_handle: usize,
}

impl<C, M, V, B, Bu, T> LLVMResourcePools<C, M, V, B, Bu, T> {
    pub fn new() -> Self {
        Self {
            contexts: None,
            modules: None,
            values: None,
            basic_blocks: None,
            builders: None,
            types: None,
            next_handle: 0,
        }
    }

    // Contexts
    pub fn create_context(&mut self, context: *mut C) -> Option<Handle> {
        let handle = Handle(self.next_handle);
        self.next_handle += 1;
        let c_pointer = CPointer::new(context)?;
        let context_map = self.contexts.get_or_insert_with(HashMap::new);
        context_map.insert(handle, Arc::new(RwLock::new(c_pointer)));
        Some(handle)
    }

    pub fn get_context(&self, handle: Handle) -> Option<Arc<RwLock<CPointer<C>>>> {
        self.contexts.as_ref()?.get(&handle).cloned()
    }

    // Modules
    pub fn create_module(&mut self, module: *mut M) -> Option<Handle> {
        let handle = Handle(self.next_handle);
        self.next_handle += 1;
        let c_pointer = CPointer::new(module)?;
        let module_map = self.modules.get_or_insert_with(HashMap::new);
        module_map.insert(handle, Arc::new(RwLock::new(c_pointer)));
        Some(handle)
    }

    pub fn get_module(&self, handle: Handle) -> Option<Arc<RwLock<CPointer<M>>>> {
        self.modules.as_ref()?.get(&handle).cloned()
    }

    // Values
    pub fn create_value(&mut self, value: *mut V) -> Option<Handle> {
        let handle = Handle(self.next_handle);
        self.next_handle += 1;
        let c_pointer = CPointer::new(value)?;
        let value_map = self.values.get_or_insert_with(HashMap::new);
        value_map.insert(handle, Arc::new(RwLock::new(c_pointer)));
        Some(handle)
    }

    pub fn get_value(&self, handle: Handle) -> Option<Arc<RwLock<CPointer<V>>>> {
        self.values.as_ref()?.get(&handle).cloned()
    }

    // Basic Blocks
    pub fn create_basic_block(&mut self, basic_block: *mut B) -> Option<Handle> {
        let handle = Handle(self.next_handle);
        self.next_handle += 1;
        let c_pointer = CPointer::new(basic_block)?;
        let basic_block_map = self.basic_blocks.get_or_insert_with(HashMap::new);
        basic_block_map.insert(handle, Arc::new(RwLock::new(c_pointer)));
        Some(handle)
    }

    pub fn get_basic_block(&self, handle: Handle) -> Option<Arc<RwLock<CPointer<B>>>> {
        self.basic_blocks.as_ref()?.get(&handle).cloned()
    }

    // Builders
    pub fn create_builder(&mut self, builder: *mut Bu) -> Option<Handle> {
        let handle = Handle(self.next_handle);
        self.next_handle += 1;
        let c_pointer = CPointer::new(builder)?;
        let builder_map = self.builders.get_or_insert_with(HashMap::new);
        builder_map.insert(handle, Arc::new(RwLock::new(c_pointer)));
        Some(handle)
    }

    pub fn get_builder(&self, handle: Handle) -> Option<Arc<RwLock<CPointer<Bu>>>> {
        self.builders.as_ref()?.get(&handle).cloned()
    }

    // Types
    pub fn create_type(&mut self, type_ref: *mut T) -> Option<Handle> {
        let handle = Handle(self.next_handle);
        self.next_handle += 1;
        let c_pointer = CPointer::new(type_ref)?;
        let type_map = self.types.get_or_insert_with(HashMap::new);
        type_map.insert(handle, Arc::new(RwLock::new(c_pointer)));
        Some(handle)
    }

    pub fn get_type(&self, handle: Handle) -> Option<Arc<RwLock<CPointer<T>>>> {
        self.types.as_ref()?.get(&handle).cloned()
    }
}
