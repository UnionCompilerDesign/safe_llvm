extern crate llvm_sys as llvm;

use std::{collections::HashMap, sync::{Arc, RwLock}};
use crate::memory_management::pointer::CPointer;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValueHandle(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BasicBlockHandle(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ContextHandle(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ModuleHandle(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BuilderHandle(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TypeHandle(usize);

pub struct LLVMResourcePools<T> {
    values: Option<HashMap<ValueHandle, Arc<RwLock<CPointer<T>>>>>,    
    basic_block: Option<HashMap<BasicBlockHandle, Arc<RwLock<CPointer<T>>>>>,
    context: Option<HashMap<ContextHandle, Arc<RwLock<CPointer<T>>>>>,
    module: Option<HashMap<ModuleHandle, Arc<RwLock<CPointer<T>>>>>,
    builder: Option<HashMap<BuilderHandle, Arc<RwLock<CPointer<T>>>>>,
    type_ref: Option<HashMap<TypeHandle, Arc<RwLock<CPointer<T>>>>>,  
    next_handle: usize,
}

impl<T> LLVMResourcePools<T> {
    pub fn new() -> Self {
        Self {
            values: None,
            basic_block: None,
            context: None,
            module: None,
            builder: None,
            type_ref: None,
            next_handle: 0,
        }
    }

    pub fn get_value(&self, handle: ValueHandle) -> Option<Arc<RwLock<CPointer<T>>>> {
        self.values.as_ref()?.get(&handle).cloned()
    }

    pub fn create_value_handle(&mut self, value: *mut T) -> Option<ValueHandle> {
        if value.is_null() {
            return None;
        }
    
        let handle = ValueHandle(self.next_handle);
        self.next_handle += 1;
    
        if let Some(c_pointer) = CPointer::new(value) {
            let map = self.values.get_or_insert_with(HashMap::new);
            map.insert(handle, Arc::new(RwLock::new(c_pointer)));
            Some(handle)
        } else {
            None
        }
    }
    

    pub fn get_basic_block(&self, handle: BasicBlockHandle) -> Option<Arc<RwLock<CPointer<T>>>> {
        self.basic_block.as_ref()?.get(&handle).cloned()
    }


    pub fn create_basic_block_handle(&mut self, basic_block: *mut T) -> Option<BasicBlockHandle> {
        if basic_block.is_null() {
            return None;
        }
    
        let handle = BasicBlockHandle(self.next_handle);
        self.next_handle += 1;
    
        if let Some(c_pointer) = CPointer::new(basic_block) {
            let map = self.basic_block.get_or_insert_with(HashMap::new);
            map.insert(handle, Arc::new(RwLock::new(c_pointer)));
            Some(handle)
        } else {
            None
        }
    }

    pub fn get_context(&self, handle: ContextHandle) -> Option<Arc<RwLock<CPointer<T>>>> {
        self.context.as_ref()?.get(&handle).cloned()
    }

    pub fn create_context_handle(&mut self, context: *mut T) -> Option<ContextHandle> {
        if context.is_null() {
            return None;
        }
    
        let handle = ContextHandle(self.next_handle);
        self.next_handle += 1;
    
        if let Some(c_pointer) = CPointer::new(context) {
            let map = self.context.get_or_insert_with(HashMap::new);
            map.insert(handle, Arc::new(RwLock::new(c_pointer)));
            Some(handle)
        } else {
            None
        }
    }

    pub fn get_module(&self, handle: ModuleHandle) -> Option<Arc<RwLock<CPointer<T>>>> {
        self.module.as_ref()?.get(&handle).cloned()
    }

    pub fn create_module_handle(&mut self, module: *mut T) -> Option<ModuleHandle> {
        if module.is_null() {
            return None;
        }
    
        let handle = ModuleHandle(self.next_handle);
        self.next_handle += 1;
    
        if let Some(c_pointer) = CPointer::new(module) {
            let map = self.module.get_or_insert_with(HashMap::new);
            map.insert(handle, Arc::new(RwLock::new(c_pointer)));
            Some(handle)
        } else {
            None
        }
    }
    

    pub fn get_builder(&self, handle: BuilderHandle) -> Option<Arc<RwLock<CPointer<T>>>> {
        self.builder.as_ref()?.get(&handle).cloned()
    }

    pub fn create_builder_handle(&mut self, builder: *mut T) -> Option<BuilderHandle> {
        if builder.is_null() {
            return None;
        }
    
        let handle = BuilderHandle(self.next_handle);
        self.next_handle += 1;
    
        if let Some(c_pointer) = CPointer::new(builder) {
            let map = self.builder.get_or_insert_with(HashMap::new);
            map.insert(handle, Arc::new(RwLock::new(c_pointer)));
            Some(handle)
        } else {
            None
        }
    }
    
    pub fn get_type_ref(&self, handle: TypeHandle) -> Option<Arc<RwLock<CPointer<T>>>> {
        self.type_ref.as_ref()?.get(&handle).cloned()
    }

    pub fn create_type_handle(&mut self, type_ref: *mut T) -> Option<TypeHandle> {
        if type_ref.is_null() {
            return None;
        }
    
        let handle = TypeHandle(self.next_handle);
        self.next_handle += 1;
    
        if let Some(c_pointer) = CPointer::new(type_ref) {
            let map = self.type_ref.get_or_insert_with(HashMap::new);
            map.insert(handle, Arc::new(RwLock::new(c_pointer)));
            Some(handle)
        } else {
            None
        }
    }
    
}
