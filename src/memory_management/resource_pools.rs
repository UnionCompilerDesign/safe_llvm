/*  
    A struct for managing resource pools for LLVM pointers using multi-threaded pointers.
    This struct provides controlled, mutable access to LLVM pointers through the usage of a tag system. 
*/

extern crate llvm_sys as llvm;

use llvm::prelude::{LLVMBasicBlockRef, LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef};

use std::{collections::HashMap, sync::{Arc, RwLock}};

use crate::memory_management::pointer::{LLVMRef, SafeLLVMPointer};

/// Each tag is unique throughout the course of an application's runtime. 

/// Gives access to context resources in the pools. 
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ContextTag(usize);

/// Gives access to module resources in the pools. 
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ModuleTag(usize);

/// Gives access to value resources in the pools. 
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValueTag(usize);

/// Gives access to value resources in the pools. 
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BasicBlockTag(usize);

/// Gives access to value resources in the pools. 
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BuilderTag(usize);

/// Gives access to value resources in the pools. 
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TypeTag(usize);

pub enum Tag {
    Context(ContextTag),
    Module(ModuleTag),
    Value(ValueTag),
    BasicBlock(BasicBlockTag),
    Builder(BuilderTag),
    Type(TypeTag),
}

pub struct ResourcePools {
    contexts: Option<HashMap<ContextTag, Arc<RwLock<SafeLLVMPointer>>>>,
    modules: Option<HashMap<ModuleTag, Arc<RwLock<SafeLLVMPointer>>>>,
    values: Option<HashMap<ValueTag, Arc<RwLock<SafeLLVMPointer>>>>,
    basic_blocks: Option<HashMap<BasicBlockTag, Arc<RwLock<SafeLLVMPointer>>>>,
    builders: Option<HashMap<BuilderTag, Arc<RwLock<SafeLLVMPointer>>>>,
    types: Option<HashMap<TypeTag, Arc<RwLock<SafeLLVMPointer>>>>,
    next_tag: usize,
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
            next_tag: 0,
        }
    }

    /// Increments the tag counter.
    fn increment_tag(&mut self) {
        self.next_tag += 1;
    }

    /// Creates a new context and stores it in the resource pools.
    pub fn store_context(&mut self, context: LLVMContextRef) -> Option<ContextTag> {
        let tag = ContextTag(self.next_tag);
        self.increment_tag();

        let c_pointer = SafeLLVMPointer::new(LLVMRef::Context(context))?;

        let context_map = self.contexts.get_or_insert_with(HashMap::new);
        context_map.insert(tag, Arc::new(RwLock::new(c_pointer)));

        Some(tag)
    }

    /// Retrieves a context from the resource pools.
    pub fn get_context(&self, tag: ContextTag) -> Option<Arc<RwLock<SafeLLVMPointer>>> {
        self.contexts.as_ref()?.get(&tag).cloned()
    }

    /// Creates a new module and stores it in the resource pools.
    pub fn store_module(&mut self, module: LLVMModuleRef) -> Option<ModuleTag> {
        let tag = ModuleTag(self.next_tag);
        self.increment_tag(); 

        let c_pointer = SafeLLVMPointer::new(LLVMRef::Module(module))?;

        let module_map = self.modules.get_or_insert_with(HashMap::new);
        module_map.insert(tag, Arc::new(RwLock::new(c_pointer)));

        Some(tag)
    }

    /// Retrieves a module from the resource pools.
    pub fn get_module(&self, tag: ModuleTag) -> Option<Arc<RwLock<SafeLLVMPointer>>> {
        self.modules.as_ref()?.get(&tag).cloned()
    }

    /// Creates a new value and stores it in the resource pools.
    pub fn store_value(&mut self, value: LLVMValueRef) -> Option<ValueTag> {
        let tag = ValueTag(self.next_tag);
        self.increment_tag();        

        let c_pointer = SafeLLVMPointer::new(LLVMRef::Value(value))?;

        let value_map = self.values.get_or_insert_with(HashMap::new);
        value_map.insert(tag, Arc::new(RwLock::new(c_pointer)));

        Some(tag)
    }

    /// Retrieves a value from the resource pools.
    pub fn get_value(&self, tag: ValueTag) -> Option<Arc<RwLock<SafeLLVMPointer>>> {
        self.values.as_ref()?.get(&tag).cloned()
    }

    /// Creates a new basic block and stores it in the resource pools.
    pub fn store_basic_block(&mut self, basic_block: LLVMBasicBlockRef) -> Option<BasicBlockTag> {
        let tag = BasicBlockTag(self.next_tag);
        self.increment_tag();        

        let c_pointer = SafeLLVMPointer::new(LLVMRef::BasicBlock(basic_block))?;

        let basic_block_map = self.basic_blocks.get_or_insert_with(HashMap::new);
        basic_block_map.insert(tag, Arc::new(RwLock::new(c_pointer)));

        Some(tag)
    }

    /// Retrieves a basic block from the resource pools.
    pub fn get_basic_block(&self, tag: BasicBlockTag) -> Option<Arc<RwLock<SafeLLVMPointer>>> {
        self.basic_blocks.as_ref()?.get(&tag).cloned()
    }

    /// Creates a new builder and stores it in the resource pools.
    pub fn store_builder(&mut self, builder: LLVMBuilderRef) -> Option<BuilderTag> {
        let tag = BuilderTag(self.next_tag);
        self.increment_tag();        

        let c_pointer = SafeLLVMPointer::new(LLVMRef::Builder(builder))?;

        let builder_map = self.builders.get_or_insert_with(HashMap::new);
        builder_map.insert(tag, Arc::new(RwLock::new(c_pointer)));

        Some(tag)
    }

    /// Retrieves a builder from the resource pools.
    pub fn get_builder(&self, tag: BuilderTag) -> Option<Arc<RwLock<SafeLLVMPointer>>> {
        self.builders.as_ref()?.get(&tag).cloned()
    }

    /// Creates a new type and stores it in the resource pools.
    pub fn store_type(&mut self, type_ref: LLVMTypeRef) -> Option<TypeTag> {
        let tag = TypeTag(self.next_tag);
        self.increment_tag();    
            
        let c_pointer = SafeLLVMPointer::new(LLVMRef::Type(type_ref))?;
        
        let type_map = self.types.get_or_insert_with(HashMap::new);
        type_map.insert(tag, Arc::new(RwLock::new(c_pointer)));

        Some(tag)
    }

    /// Retrieves a type from the resource pools.
    pub fn get_type(&self, tag: TypeTag) -> Option<Arc<RwLock<SafeLLVMPointer>>> {
        self.types.as_ref()?.get(&tag).cloned()
    }
}
