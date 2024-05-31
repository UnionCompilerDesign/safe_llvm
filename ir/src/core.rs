extern crate llvm_sys as llvm;

use llvm::{core, prelude::{LLVMBasicBlockRef, LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef}};

use std::{collections::HashMap, ffi::CString, sync::{Arc, RwLock}};

use crate::common::pointer::{LLVMRef, LLVMRefType, SafeLLVMPointer};

/// Each tag is unique throughout the course of an application's runtime. 
pub enum Tag {
    Context(ContextTag),
    Module(ModuleTag),
    Value(ValueTag),
    BasicBlock(BasicBlockTag),
    Builder(BuilderTag),
    Type(TypeTag),
}


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

#[derive(Clone)]
pub struct EnumDefinition {
    name: String,
    variant_mapping: HashMap<String, i64>, 
}

impl EnumDefinition {
    pub fn new(name: String, variant_mapping: HashMap<String, i64>) -> Self {
        Self {
            name,
            variant_mapping,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_variant(&self, name: &str) -> Option<i64> {
        self.variant_mapping.iter().find_map(|(var_name, value)| {
            if var_name == name {
                Some(*value)
            } else {
                None
            }
        })
    }
}


pub struct IRGenerator {
    contexts: Option<HashMap<ContextTag, Arc<RwLock<SafeLLVMPointer>>>>,
    modules: Option<HashMap<ModuleTag, Arc<RwLock<SafeLLVMPointer>>>>,
    values: Option<HashMap<ValueTag, Arc<RwLock<SafeLLVMPointer>>>>,
    basic_blocks: Option<HashMap<BasicBlockTag, Arc<RwLock<SafeLLVMPointer>>>>,
    basic_block_tag_map: Option<HashMap<LLVMBasicBlockRef, BasicBlockTag>>,
    builders: Option<HashMap<BuilderTag, Arc<RwLock<SafeLLVMPointer>>>>,
    types: Option<HashMap<TypeTag, Arc<RwLock<SafeLLVMPointer>>>>,
    enums: Option<HashMap<TypeTag, EnumDefinition>>,
    next_tag: usize,
}

impl IRGenerator {
    /// Constructs a new `IRGenerator` instance.
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

    fn store_basic_block_tag(&mut self, basic_block: LLVMBasicBlockRef, tag: BasicBlockTag) {
        let block_map = self.basic_block_tag_map.get_or_insert_with(HashMap::new);
        block_map.insert(basic_block, tag);
    }

    fn retrieve_basic_block_tag(&mut self, basic_block: LLVMBasicBlockRef) -> Option<BasicBlockTag> {
        let block_map = self.basic_block_tag_map.get_or_insert_with(HashMap::new);
        block_map.get(&basic_block).cloned()
    }

    /// Creates a new basic block and stores it in the resource pools.
    pub fn store_basic_block(&mut self, basic_block: LLVMBasicBlockRef) -> Option<BasicBlockTag> {
        let tag = BasicBlockTag(self.next_tag);
        self.increment_tag();        

        self.store_basic_block_tag(basic_block.clone(), tag.clone());

        let c_pointer = SafeLLVMPointer::new(LLVMRef::BasicBlock(basic_block))?;

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

    pub fn store_enum_definition(&mut self, tag: TypeTag, enum_definition: EnumDefinition) {
        let enums_map = self.enums.get_or_insert(HashMap::new());
        enums_map.insert(tag, enum_definition);
    }

    pub fn get_enum_definition(&self, tag: TypeTag) -> Option<EnumDefinition> {
        self.enums.as_ref()?.get(&tag).cloned()
    }

    /// Allocates a new LLVM context and stores it in the resource pool.
    pub fn create_context(&mut self) -> Option<ContextTag> {
        let raw_ptr: LLVMContextRef = unsafe { core::LLVMContextCreate() };

        if raw_ptr.is_null() {
            return None;
        }

        self.store_context(raw_ptr)
    }

    /// Allocates a new LLVM module in a specified context and stores it in the resource pool.
    pub fn create_module(&mut self, module_name: &str, context_tag: ContextTag) -> Option<ModuleTag> {
        let c_module_name: CString = CString::new(module_name).expect("Failed to create CString from module name");

        let context_arc_rwlock = self.get_context(context_tag)?;
        
        let context_rwlock = context_arc_rwlock.read().expect("Failed to lock context for reading");

        let context_ptr = context_rwlock.read(LLVMRefType::Context, |context_ref| {
            if let LLVMRef::Context(ptr) = context_ref {
                Some(*ptr)  
            } else {
                return None;
            }
        })?;

        let module_ptr: LLVMModuleRef = unsafe {
            core::LLVMModuleCreateWithNameInContext(c_module_name.as_ptr(), context_ptr) 
        }; 

        if module_ptr.is_null() {
            return None;
        }

        self.store_module(module_ptr)
    }
}
