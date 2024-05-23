/*  
    A struct for managing resource pools for LLVM pointers using multi-threaded pointers.
    This struct provides controlled, mutable access to LLVM pointers through the usage of a tag system. 
*/

extern crate llvm_sys as llvm;

use llvm::prelude::{LLVMBasicBlockRef, LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef};

use std::{collections::HashMap, hash::Hash, hash::Hasher, sync::{Arc, RwLock}};

use crate::memory_management::pointer::{LLVMRef, CPointer};

use super::definitions::EnumDefinition;

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

#[derive(Clone, Debug)]

pub struct BasicBlockMapTag{
    basic_block_ref: Arc<RwLock<LLVMBasicBlockRef>>
}

impl BasicBlockMapTag {
    pub fn new(reference: LLVMBasicBlockRef) -> Self {
        Self {
            basic_block_ref: Arc::new(RwLock::new(reference))
        }
    }
}

impl PartialEq for BasicBlockMapTag {
    fn eq(&self, other: &Self) -> bool {
        let self_reference: LLVMBasicBlockRef = *self.basic_block_ref.read().expect("Couldn't read other BB map tag!");
        let other_reference: LLVMBasicBlockRef = *other.basic_block_ref.read().expect("Couldn't read other BB map tag!");

        self_reference == other_reference
    }
}

impl Eq for BasicBlockMapTag {}

impl Hash for BasicBlockMapTag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let self_reference: LLVMBasicBlockRef = *self.basic_block_ref.read().expect("Couldn't read other BB map tag!");
        self_reference.hash(state);
    }
}
pub enum Tag {
    Context(ContextTag),
    Module(ModuleTag),
    Value(ValueTag),
    BasicBlock(BasicBlockTag),
    Builder(BuilderTag),
    Type(TypeTag),
}

pub struct ResourcePools {
    contexts: Option<HashMap<ContextTag, Arc<RwLock<CPointer>>>>,
    modules: Option<HashMap<ModuleTag, Arc<RwLock<CPointer>>>>,
    values: Option<HashMap<ValueTag, Arc<RwLock<CPointer>>>>,
    basic_blocks: Option<HashMap<BasicBlockTag, Arc<RwLock<CPointer>>>>,
    basic_block_tag_map: Option<HashMap<BasicBlockMapTag, BasicBlockTag>>,
    builders: Option<HashMap<BuilderTag, Arc<RwLock<CPointer>>>>,
    types: Option<HashMap<TypeTag, Arc<RwLock<CPointer>>>>,
    enums: Option<HashMap<TypeTag, EnumDefinition>>,
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
            basic_block_tag_map: None,
            builders: None,
            types: None,
            enums: None,
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

        let c_pointer = CPointer::new(LLVMRef::Context(context))?;

        let context_map = self.contexts.get_or_insert_with(HashMap::new);
        context_map.insert(tag, Arc::new(RwLock::new(c_pointer)));

        Some(tag)
    }

    /// Retrieves a context from the resource pools.
    pub fn get_context(&self, tag: ContextTag) -> Option<Arc<RwLock<CPointer>>> {
        self.contexts.as_ref()?.get(&tag).cloned()
    }

    /// Creates a new module and stores it in the resource pools.
    pub fn store_module(&mut self, module: LLVMModuleRef) -> Option<ModuleTag> {
        let tag = ModuleTag(self.next_tag);
        self.increment_tag(); 

        let c_pointer = CPointer::new(LLVMRef::Module(module))?;

        let module_map = self.modules.get_or_insert_with(HashMap::new);
        module_map.insert(tag, Arc::new(RwLock::new(c_pointer)));

        Some(tag)
    }

    /// Retrieves a module from the resource pools.
    pub fn get_module(&self, tag: ModuleTag) -> Option<Arc<RwLock<CPointer>>> {
        self.modules.as_ref()?.get(&tag).cloned()
    }

    /// Creates a new value and stores it in the resource pools.
    pub fn store_value(&mut self, value: LLVMValueRef) -> Option<ValueTag> {
        let tag = ValueTag(self.next_tag);
        self.increment_tag();        

        let c_pointer = CPointer::new(LLVMRef::Value(value))?;

        let value_map = self.values.get_or_insert_with(HashMap::new);
        value_map.insert(tag, Arc::new(RwLock::new(c_pointer)));

        Some(tag)
    }

    /// Retrieves a value from the resource pools.
    pub fn get_value(&self, tag: ValueTag) -> Option<Arc<RwLock<CPointer>>> {
        self.values.as_ref()?.get(&tag).cloned()
    }

    fn store_basic_block_tag(&mut self, basic_block: LLVMBasicBlockRef, tag: BasicBlockTag) {
        let block_map = self.basic_block_tag_map.get_or_insert_with(HashMap::new);
        let block_representation = BasicBlockMapTag::new(basic_block);
        block_map.insert(block_representation, tag);
    }

    fn retrieve_basic_block_tag(&mut self, basic_block: LLVMBasicBlockRef) -> Option<BasicBlockTag> {
        let block_map = self.basic_block_tag_map.get_or_insert_with(HashMap::new);
        let block_representation = BasicBlockMapTag::new(basic_block);
        block_map.get(&block_representation).cloned()
    }

    /// Creates a new basic block and stores it in the resource pools.
    pub fn store_basic_block(&mut self, basic_block: LLVMBasicBlockRef) -> Option<BasicBlockTag> {
        let tag = BasicBlockTag(self.next_tag);
        self.increment_tag();        

        self.store_basic_block_tag(basic_block.clone(), tag.clone());

        let c_pointer = CPointer::new(LLVMRef::BasicBlock(basic_block))?;

        let basic_block_map = self.basic_blocks.get_or_insert_with(HashMap::new);
        basic_block_map.insert(tag, Arc::new(RwLock::new(c_pointer)));

        Some(tag)
    }

    /// Gets a basic block's tag from pools
    pub fn get_basic_block_tag(&mut self, basic_block: LLVMBasicBlockRef) -> Option<BasicBlockTag> {      

        let tag = self.retrieve_basic_block_tag(basic_block);

        tag
    }

    /// Retrieves a basic block from the resource pools.
    pub fn get_basic_block(&self, tag: BasicBlockTag) -> Option<Arc<RwLock<CPointer>>> {
        self.basic_blocks.as_ref()?.get(&tag).cloned()
    }

    /// Creates a new builder and stores it in the resource pools.
    pub fn store_builder(&mut self, builder: LLVMBuilderRef) -> Option<BuilderTag> {
        let tag = BuilderTag(self.next_tag);
        self.increment_tag();        

        let c_pointer = CPointer::new(LLVMRef::Builder(builder))?;

        let builder_map = self.builders.get_or_insert_with(HashMap::new);
        builder_map.insert(tag, Arc::new(RwLock::new(c_pointer)));

        Some(tag)
    }

    /// Retrieves a builder from the resource pools.
    pub fn get_builder(&self, tag: BuilderTag) -> Option<Arc<RwLock<CPointer>>> {
        self.builders.as_ref()?.get(&tag).cloned()
    }

    /// Creates a new type and stores it in the resource pools.
    pub fn store_type(&mut self, type_ref: LLVMTypeRef) -> Option<TypeTag> {
        let tag = TypeTag(self.next_tag);
        self.increment_tag();    
            
        let c_pointer = CPointer::new(LLVMRef::Type(type_ref))?;
        
        let type_map = self.types.get_or_insert_with(HashMap::new);
        type_map.insert(tag, Arc::new(RwLock::new(c_pointer)));

        Some(tag)
    }

    /// Retrieves a type from the resource pools.
    pub fn get_type(&self, tag: TypeTag) -> Option<Arc<RwLock<CPointer>>> {
        self.types.as_ref()?.get(&tag).cloned()
    }

    pub fn store_enum_definition(&mut self, tag: TypeTag, enum_definition: EnumDefinition) {
        let enums_map = self.enums.get_or_insert(HashMap::new());
        enums_map.insert(tag, enum_definition);
    }

    pub fn get_enum_definition(&self, tag: TypeTag) -> Option<EnumDefinition> {
        self.enums.as_ref()?.get(&tag).cloned()
    }
}
