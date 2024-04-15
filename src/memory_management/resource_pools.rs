extern crate llvm_sys as llvm;

use std::{collections::HashMap, sync::{Arc, RwLock}};
use crate::memory_management::ir_pointer::IRPointer;

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
    values: Option<HashMap<ValueHandle, Arc<RwLock<IRPointer<T>>>>>,    
    basic_block: Option<HashMap<BasicBlockHandle, Arc<RwLock<IRPointer<T>>>>>,
    context: Option<HashMap<ContextHandle, Arc<RwLock<IRPointer<T>>>>>,
    module: Option<HashMap<ModuleHandle, Arc<RwLock<IRPointer<T>>>>>,
    builder: Option<HashMap<BuilderHandle, Arc<RwLock<IRPointer<T>>>>>,
    type_ref: Option<HashMap<TypeHandle, Arc<RwLock<IRPointer<T>>>>>,  
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

    pub fn get_value(&self, handle: ValueHandle) -> Option<Arc<RwLock<IRPointer<T>>>> {
        self.values.as_ref()?.get(&handle).cloned()
    }

    pub fn create_value_handle(&mut self, value: *mut T) -> ValueHandle {
        let handle: ValueHandle = ValueHandle(self.next_handle);
        self.next_handle += 1;
        let pointer: Arc<RwLock<IRPointer<T>>> = Arc::new(RwLock::new(IRPointer::new(Some(value))));
        self.values.get_or_insert_with(HashMap::new).insert(handle, pointer);
        handle
    }

    pub fn get_basic_block(&self, handle: BasicBlockHandle) -> Option<Arc<RwLock<IRPointer<T>>>> {
        self.basic_block.as_ref()?.get(&handle).cloned()
    }

    pub fn create_basic_block_handle(&mut self, basic_block: *mut T) -> BasicBlockHandle {
        let handle: BasicBlockHandle = BasicBlockHandle(self.next_handle);
        self.next_handle += 1;
        let pointer: Arc<RwLock<IRPointer<T>>> = Arc::new(RwLock::new(IRPointer::new(Some(basic_block))));
        self.basic_block.get_or_insert_with(HashMap::new).insert(handle, pointer);
        handle
    }

    pub fn get_context(&self, handle: ContextHandle) -> Option<Arc<RwLock<IRPointer<T>>>> {
        self.context.as_ref()?.get(&handle).cloned()
    }

    pub fn create_context_handle(&mut self, context: *mut T) -> ContextHandle {
        let handle: ContextHandle = ContextHandle(self.next_handle);
        self.next_handle += 1;
        let pointer: Arc<RwLock<IRPointer<T>>> = Arc::new(RwLock::new(IRPointer::new(Some(context))));
        self.context.get_or_insert_with(HashMap::new).insert(handle, pointer);
        handle
    }

    pub fn get_module(&self, handle: ModuleHandle) -> Option<Arc<RwLock<IRPointer<T>>>> {
        self.module.as_ref()?.get(&handle).cloned()
    }

    pub fn create_module_handle(&mut self, module: *mut T) -> ModuleHandle {
        let handle: ModuleHandle = ModuleHandle(self.next_handle);
        self.next_handle += 1;
        let pointer: Arc<RwLock<IRPointer<T>>> = Arc::new(RwLock::new(IRPointer::new(Some(module))));
        self.module.get_or_insert_with(HashMap::new).insert(handle, pointer);
        handle
    }

    pub fn get_builder(&self, handle: BuilderHandle) -> Option<Arc<RwLock<IRPointer<T>>>> {
        self.builder.as_ref()?.get(&handle).cloned()
    }

    pub fn create_builder_handle(&mut self, builder: *mut T) -> BuilderHandle {
        let handle: BuilderHandle = BuilderHandle(self.next_handle);
        self.next_handle += 1;
        let pointer: Arc<RwLock<IRPointer<T>>> = Arc::new(RwLock::new(IRPointer::new(Some(builder))));
        self.builder.get_or_insert_with(HashMap::new).insert(handle, pointer);
        handle
    }

    pub fn get_type_ref(&self, handle: TypeHandle) -> Option<Arc<RwLock<IRPointer<T>>>> {
        self.type_ref.as_ref()?.get(&handle).cloned()
    }

    pub fn create_type_handle(&mut self, type_ref: *mut T) -> TypeHandle {
        let handle: TypeHandle = TypeHandle(self.next_handle);
        self.next_handle += 1;
        let pointer: Arc<RwLock<IRPointer<T>>> = Arc::new(RwLock::new(IRPointer::new(Some(type_ref))));
        self.type_ref.get_or_insert_with(HashMap::new).insert(handle, pointer);
        handle
    }
}
