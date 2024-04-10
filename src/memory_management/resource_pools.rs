extern crate llvm_sys as llvm;

use std::collections::HashMap;

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
    values: Option<HashMap<ValueHandle, IRPointer<T>>>,
    basic_block: Option<HashMap<BasicBlockHandle, IRPointer<T>>>,
    context: Option<HashMap<ContextHandle, IRPointer<T>>>,
    module: Option<HashMap<ModuleHandle, IRPointer<T>>>,
    builder: Option<HashMap<BuilderHandle, IRPointer<T>>>,
    type_ref: Option<HashMap<TypeHandle, IRPointer<T>>>,  
    next_handle: usize, // Generates unique IDs
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

    pub fn get_value(&self, handle: ValueHandle) -> Option<&IRPointer<T>> {
        match &self.values {
            Some(values) => {
                values.get(&handle)
            }
            None => None,
        }
    }

    pub fn create_value_handle(&mut self, value: *mut T) -> ValueHandle {
        let handle = ValueHandle(self.next_handle);
        self.next_handle += 1;

        let pointer = IRPointer::new(value);
        if let Some(values) = self.values.as_mut() {
            values.insert(handle, pointer);
        } else {
            let mut map = HashMap::new();
            map.insert(handle, pointer);
            self.values = Some(map);
        }

        handle
    }


    pub fn get_basic_block(&self, handle: BasicBlockHandle) -> Option<&IRPointer<T>> {
        match &self.basic_block {
            Some(basic_blocks) => {
                basic_blocks.get(&handle)
            }
            None => None,
        }
    }

    pub fn create_basic_block_handle(&mut self, basic_block: *mut T) -> BasicBlockHandle {
        let handle = BasicBlockHandle(self.next_handle);
        self.next_handle += 1;

        let pointer = IRPointer::new(basic_block);
        if let Some(basic_blocks) = self.basic_block.as_mut() {
            basic_blocks.insert(handle, pointer);
        } else {
            let mut map = HashMap::new();
            map.insert(handle, pointer);
            self.basic_block = Some(map);
        }

        handle
    }
    
    pub fn get_context(&self, handle: ContextHandle) -> Option<&IRPointer<T>> {
        self.context.as_ref()?.get(&handle)
    }

    pub fn create_context_handle(&mut self, context: *mut T) -> ContextHandle {
        let handle = ContextHandle(self.next_handle);
        self.next_handle += 1;

        let pointer = IRPointer::new(context);
        self.context.get_or_insert_with(HashMap::new).insert(handle, pointer);
        handle
    }

    pub fn get_module(&self, handle: ModuleHandle) -> Option<&IRPointer<T>> {
        self.module.as_ref()?.get(&handle)
    }

    pub fn create_module_handle(&mut self, module: *mut T) -> ModuleHandle {
        let handle = ModuleHandle(self.next_handle);
        self.next_handle += 1;

        let pointer = IRPointer::new(module);
        self.module.get_or_insert_with(HashMap::new).insert(handle, pointer);
        handle
    }

    pub fn get_builder(&self, handle: BuilderHandle) -> Option<&IRPointer<T>> {
        self.builder.as_ref()?.get(&handle)
    }

    pub fn create_builder_handle(&mut self, builder: *mut T) -> BuilderHandle {
        let handle = BuilderHandle(self.next_handle);
        self.next_handle += 1;

        let pointer = IRPointer::new(builder);
        self.builder.get_or_insert_with(HashMap::new).insert(handle, pointer);
        handle
    }

    pub fn get_type_ref(&self, handle: TypeHandle) -> Option<&IRPointer<T>> {
        self.type_ref.as_ref()?.get(&handle)
    }

    pub fn create_type_handle(&mut self, type_ref: *mut T) -> TypeHandle {
        let handle = TypeHandle(self.next_handle);
        self.next_handle += 1;

        let pointer = IRPointer::new(type_ref);
        self.type_ref.get_or_insert_with(HashMap::new).insert(handle, pointer);
        handle
    }
}
