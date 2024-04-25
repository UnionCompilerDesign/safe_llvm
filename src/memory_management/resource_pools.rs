/*  
    A struct for managing resource pools for LLVM pointers using multi-threaded pointers.
    This struct provides controlled, mutable access to LLVM pointers. 
*/

extern crate llvm_sys as llvm;

use llvm::prelude::{LLVMBasicBlockRef, LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef};

use std::{collections::HashMap, sync::{Arc, RwLock}};

use crate::memory_management::pointer::{LLVMRef, CPointer};

/// Each handle is unique throughout the course of an application's runtime. 

/// Gives access to context resources in the pools. 
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ContextHandle(usize);

/// Gives access to module resources in the pools. 
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ModuleHandle(usize);

/// Gives access to value resources in the pools. 
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValueHandle(usize);

/// Gives access to value resources in the pools. 
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BasicBlockHandle(usize);

/// Gives access to value resources in the pools. 
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BuilderHandle(usize);

/// Gives access to value resources in the pools. 
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TypeHandle(usize);

pub struct ResourcePools {
    contexts: Option<HashMap<ContextHandle, Arc<RwLock<CPointer>>>>,
    modules: Option<HashMap<ModuleHandle, Arc<RwLock<CPointer>>>>,
    values: Option<HashMap<ValueHandle, Arc<RwLock<CPointer>>>>,
    basic_blocks: Option<HashMap<BasicBlockHandle, Arc<RwLock<CPointer>>>>,
    builders: Option<HashMap<BuilderHandle, Arc<RwLock<CPointer>>>>,
    types: Option<HashMap<TypeHandle, Arc<RwLock<CPointer>>>>,
    next_handle: usize,
}

impl ResourcePools {
    /// Constructs a new `ResourcePools` instance.
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

    /// Increments the handle counter.
    fn increment_handle(&mut self) {
        self.next_handle += 1;
    }

    /// Creates a new context and stores it in the resource pools.
    pub fn store_context(&mut self, context: LLVMContextRef) -> Option<ContextHandle> {
        let handle = ContextHandle(self.next_handle);
        self.increment_handle();

        let c_pointer = CPointer::new(LLVMRef::Context(context))?;

        let context_map = self.contexts.get_or_insert_with(HashMap::new);
        context_map.insert(handle, Arc::new(RwLock::new(c_pointer)));

        Some(handle)
    }

    /// Retrieves a context from the resource pools.
    pub fn get_context(&self, handle: ContextHandle) -> Option<Arc<RwLock<CPointer>>> {
        self.contexts.as_ref()?.get(&handle).cloned()
    }

    /// Creates a new module and stores it in the resource pools.
    pub fn store_module(&mut self, module: LLVMModuleRef) -> Option<ModuleHandle> {
        let handle = ModuleHandle(self.next_handle);
        self.increment_handle(); 

        let c_pointer = CPointer::new(LLVMRef::Module(module))?;

        let module_map = self.modules.get_or_insert_with(HashMap::new);
        module_map.insert(handle, Arc::new(RwLock::new(c_pointer)));

        Some(handle)
    }

    /// Retrieves a module from the resource pools.
    pub fn get_module(&self, handle: ModuleHandle) -> Option<Arc<RwLock<CPointer>>> {
        self.modules.as_ref()?.get(&handle).cloned()
    }

    /// Creates a new value and stores it in the resource pools.
    pub fn store_value(&mut self, value: LLVMValueRef) -> Option<ValueHandle> {
        let handle = ValueHandle(self.next_handle);
        self.increment_handle();        

        let c_pointer = CPointer::new(LLVMRef::Value(value))?;

        let value_map = self.values.get_or_insert_with(HashMap::new);
        value_map.insert(handle, Arc::new(RwLock::new(c_pointer)));

        Some(handle)
    }

    /// Retrieves a value from the resource pools.
    pub fn get_value(&self, handle: ValueHandle) -> Option<Arc<RwLock<CPointer>>> {
        self.values.as_ref()?.get(&handle).cloned()
    }

    /// Creates a new basic block and stores it in the resource pools.
    pub fn store_basic_block(&mut self, basic_block: LLVMBasicBlockRef) -> Option<BasicBlockHandle> {
        let handle = BasicBlockHandle(self.next_handle);
        self.increment_handle();        

        let c_pointer = CPointer::new(LLVMRef::BasicBlock(basic_block))?;

        let basic_block_map = self.basic_blocks.get_or_insert_with(HashMap::new);
        basic_block_map.insert(handle, Arc::new(RwLock::new(c_pointer)));

        Some(handle)
    }

    /// Retrieves a basic block from the resource pools.
    pub fn get_basic_block(&self, handle: BasicBlockHandle) -> Option<Arc<RwLock<CPointer>>> {
        self.basic_blocks.as_ref()?.get(&handle).cloned()
    }

    /// Creates a new builder and stores it in the resource pools.
    pub fn store_builder(&mut self, builder: LLVMBuilderRef) -> Option<BuilderHandle> {
        let handle = BuilderHandle(self.next_handle);
        self.increment_handle();        

        let c_pointer = CPointer::new(LLVMRef::Builder(builder))?;

        let builder_map = self.builders.get_or_insert_with(HashMap::new);
        builder_map.insert(handle, Arc::new(RwLock::new(c_pointer)));

        Some(handle)
    }

    /// Retrieves a builder from the resource pools.
    pub fn get_builder(&self, handle: BuilderHandle) -> Option<Arc<RwLock<CPointer>>> {
        self.builders.as_ref()?.get(&handle).cloned()
    }

    /// Creates a new type and stores it in the resource pools.
    pub fn store_type(&mut self, type_ref: LLVMTypeRef) -> Option<TypeHandle> {
        let handle = TypeHandle(self.next_handle);
        self.increment_handle();    
            
        let c_pointer = CPointer::new(LLVMRef::Type(type_ref))?;
        
        let type_map = self.types.get_or_insert_with(HashMap::new);
        type_map.insert(handle, Arc::new(RwLock::new(c_pointer)));

        Some(handle)
    }

    /// Retrieves a type from the resource pools.
    pub fn get_type(&self, handle: TypeHandle) -> Option<Arc<RwLock<CPointer>>> {
        self.types.as_ref()?.get(&handle).cloned()
    }
}
