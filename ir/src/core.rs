//! # Core Utilities for LLVM IR Generation in SafeLLVM
//!
//! The core API module for SafeLLVM provides an interanl interface for managing LLVM's Intermediate
//! Representation (IR).

extern crate llvm_sys as llvm;
use llvm::{core, prelude::{LLVMBasicBlockRef, LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMTypeRef, LLVMValueRef}};
use std::{collections::HashMap, ffi::CString, sync::{Arc, RwLock}};
use common::pointer::{LLVMRef, LLVMRefType, SafeLLVMPointer};

/// Represents a definition for an LLVM enum type, mapping string names to integer values.
/// This structure aids in managing enum representations within the LLVM IR.
#[derive(Clone)]
pub struct EnumDefinition {
    name: String,
    variant_mapping: HashMap<String, i64>, 
}

/// Tag associated with an LLVM Context.
/// Provides access to stored context resources within the IRManager pools.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ContextTag(usize);

/// Tag associated with an LLVM Module.
/// Allows for retrieval and management of module resources in the IRManager pools.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ModuleTag(usize);

/// Tag associated with an LLVM Value.
/// Used for accessing and manipulating value resources in the IRManager pools.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ValueTag(usize);

/// Tag associated with an LLVM Basic Block.
/// Facilitates the retrieval and management of basic block resources in the pools.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BasicBlockTag(usize);

/// Tag associated with an LLVM Builder.
/// Used to access builder resources stored within the IRManager pools.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BuilderTag(usize);

/// Tag associated with an LLVM Type.
/// Enables the handling and retrieval of type resources from the IRManager pools.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TypeTag(usize);

impl EnumDefinition {
    /// Constructs a new EnumDefinition.
    ///
    /// # Arguments
    /// * `name` - The name of the enum.
    /// * `variant_mapping` - A mapping from the name of each variant to its integer value.
    ///
    /// # Returns
    /// A new instance of `EnumDefinition`.
    pub fn new(name: String, variant_mapping: HashMap<String, i64>) -> Self {
        Self {
            name,
            variant_mapping,
        }
    }

    /// Retrieves the name of the enum.
    ///
    /// # Returns
    /// A string slice representing the name of the enum.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Retrieves the integer value associated with a variant name, if it exists.
    ///
    /// # Arguments
    /// * `name` - The name of the variant to retrieve.
    ///
    /// # Returns
    /// An option containing the integer value of the variant, or `None` if the variant does not exist.
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


/// Enum representing unique tags for various types of LLVM objects managed within the IRManager.
/// Each tag is unique and provides a way to retrieve specific LLVM objects from internal resource pools.
pub enum Tag {
    /// Tag for identifying and retrieving LLVM `Context` objects from the IRManager's resource pools.
    /// Contexts represent environments in which LLVM IR generation and manipulation occur.
    Context(ContextTag),

    /// Tag for identifying and managing LLVM `Module` objects within the resource pools.
    /// Modules in LLVM serve as containers for function definitions and global variables.
    Module(ModuleTag),

    /// Tag used for managing LLVM `Value` objects, which represent the SSA (Static Single Assignment)
    /// values computed by instructions or the function parameters.
    Value(ValueTag),

    /// Tag associated with LLVM `BasicBlock` objects. Basic blocks are sequences of instructions
    /// without any branches except into the entry and out of the exit of the block.
    BasicBlock(BasicBlockTag),

    /// Tag for LLVM `Builder` objects, which are used to create and insert new instructions into
    /// basic blocks.
    Builder(BuilderTag),

    /// Tag related to LLVM `Type` objects, which define the data layout and type of values
    /// used in LLVM IR. This includes primitive data types (like integers and floating points),
    /// and derived types (like arrays and pointers).
    Type(TypeTag),
}

/// Core structure for managing IR generation. This includes creation, storage, and retrieval of LLVM related objects.
pub struct IRManager {
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

impl IRManager {
    /// Constructs a new instance of `IRManager`.
    ///
    /// # Returns
    /// A new `IRManager` instance with empty pools and a tag counter initialized to zero.
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

    /// Increments the tag counter by one.
    fn increment_tag(&mut self) {
        self.next_tag += 1;
    }

    /// Creates a new context and stores it in the resource pools.
    ///
    /// # Arguments
    /// * `context` - A raw pointer to an LLVMContextRef that needs to be stored.
    ///
    /// # Returns
    /// An option containing the newly created `ContextTag` if successful, `None` otherwise.
    pub fn store_context(&mut self, context: LLVMContextRef) -> Option<ContextTag> {
        let tag = ContextTag(self.next_tag);
        self.increment_tag();

        let c_pointer = SafeLLVMPointer::new(LLVMRef::Context(context))?;
        let context_map = self.contexts.get_or_insert_with(HashMap::new);
        context_map.insert(tag, Arc::new(RwLock::new(c_pointer)));

        Some(tag)
    }

    /// Retrieves a context by its tag from the resource pools.
    ///
    /// # Arguments
    /// * `tag` - The `ContextTag` used to identify the context.
    ///
    /// # Returns
    /// An option containing an `Arc<RwLock<SafeLLVMPointer>>` to the context if found, `None` otherwise.
    pub fn get_context(&self, tag: ContextTag) -> Option<Arc<RwLock<SafeLLVMPointer>>> {
        self.contexts.as_ref()?.get(&tag).cloned()
    }

    /// Creates a new module and stores it in the resource pools.
    ///
    /// # Arguments
    /// * `module` - A raw pointer to an LLVMModuleRef that needs to be stored.
    ///
    /// # Returns
    /// An option containing the newly created `ModuleTag` if successful, `None` otherwise.
    pub fn store_module(&mut self, module: LLVMModuleRef) -> Option<ModuleTag> {
        let tag = ModuleTag(self.next_tag);
        self.increment_tag();

        let c_pointer = SafeLLVMPointer::new(LLVMRef::Module(module))?;
        let module_map = self.modules.get_or_insert_with(HashMap::new);
        module_map.insert(tag, Arc::new(RwLock::new(c_pointer)));

        Some(tag)
    }

    /// Retrieves a module by its tag from the resource pools.
    ///
    /// # Arguments
    /// * `tag` - The `ModuleTag` used to identify the module.
    ///
    /// # Returns
    /// An option containing an `Arc<RwLock<SafeLLVMPointer>>` to the module if found, `None` otherwise.
    pub fn get_module(&self, tag: ModuleTag) -> Option<Arc<RwLock<SafeLLVMPointer>>> {
        self.modules.as_ref()?.get(&tag).cloned()
    }

    /// Creates a new value and stores it in the resource pools.
    ///
    /// # Arguments
    /// * `value` - A raw pointer to an LLVMValueRef that needs to be stored.
    ///
    /// # Returns
    /// An option containing the newly created `ValueTag` if successful, `None` otherwise.
    pub fn store_value(&mut self, value: LLVMValueRef) -> Option<ValueTag> {
        let tag = ValueTag(self.next_tag);
        self.increment_tag();

        let c_pointer = SafeLLVMPointer::new(LLVMRef::Value(value))?;
        let value_map = self.values.get_or_insert_with(HashMap::new);
        value_map.insert(tag, Arc::new(RwLock::new(c_pointer)));

        Some(tag)
    }

    /// Retrieves a value by its tag from the resource pools.
    ///
    /// # Arguments
    /// * `tag` - The `ValueTag` used to identify the value.
    ///
    /// # Returns
    /// An option containing an `Arc<RwLock<SafeLLVMPointer>>` to the value if found, `None` otherwise.
    pub fn get_value(&self, tag: ValueTag) -> Option<Arc<RwLock<SafeLLVMPointer>>> {
        self.values.as_ref()?.get(&tag).cloned()
    }

    /// Creates a new basic block and stores it in the resource pools.
    ///
    /// # Arguments
    /// * `basic_block` - A raw pointer to an LLVMBasicBlockRef that needs to be stored.
    ///
    /// # Returns
    /// An option containing the newly created `BasicBlockTag` if successful, `None` otherwise.
    pub fn store_basic_block(&mut self, basic_block: LLVMBasicBlockRef) -> Option<BasicBlockTag> {
        let tag = BasicBlockTag(self.next_tag);
        self.increment_tag();        

        self.store_basic_block_tag(basic_block.clone(), tag.clone());

        let c_pointer = SafeLLVMPointer::new(LLVMRef::BasicBlock(basic_block))?;

        let basic_block_map = self.basic_blocks.get_or_insert_with(HashMap::new);
        basic_block_map.insert(tag, Arc::new(RwLock::new(c_pointer)));

        Some(tag)
    }

    /// Stores a tag associated with a basic block.
    ///
    /// # Arguments
    /// * `basic_block` - A reference to an LLVMBasicBlockRef.
    /// * `tag` - The BasicBlockTag to associate with the basic block.
    fn store_basic_block_tag(&mut self, basic_block: LLVMBasicBlockRef, tag: BasicBlockTag) {
        let block_map = self.basic_block_tag_map.get_or_insert_with(HashMap::new);
        block_map.insert(basic_block, tag);
    }

    /// Retrieves a tag associated with a basic block if it exists.
    ///
    /// # Arguments
    /// * `basic_block` - A reference to an LLVMBasicBlockRef.
    ///
    /// # Returns
    /// An option containing the BasicBlockTag if found, `None` otherwise.
    fn retrieve_basic_block_tag(&mut self, basic_block: LLVMBasicBlockRef) -> Option<BasicBlockTag> {
        let block_map = self.basic_block_tag_map.get_or_insert_with(HashMap::new);
        block_map.get(&basic_block).cloned()
    }

    /// Retrieves a basic block's tag from the resource pools.
    ///
    /// # Arguments
    /// * `basic_block` - A reference to an LLVMBasicBlockRef.
    ///
    /// # Returns
    /// An option containing the BasicBlockTag associated with the basic block if found, `None` otherwise.
    pub fn get_basic_block_tag(&mut self, basic_block: LLVMBasicBlockRef) -> Option<BasicBlockTag> {      
        self.retrieve_basic_block_tag(basic_block)
    }

    /// Retrieves a basic block by its tag from the resource pools.
    ///
    /// # Arguments
    /// * `tag` - The `BasicBlockTag` used to identify the basic block.
    ///
    /// # Returns
    /// An option containing an `Arc<RwLock<SafeLLVMPointer>>` to the basic block if found, `None` otherwise.
    pub fn get_basic_block(&self, tag: BasicBlockTag) -> Option<Arc<RwLock<SafeLLVMPointer>>> {
        self.basic_blocks.as_ref()?.get(&tag).cloned()
    }

    /// Creates a new builder and stores it in the resource pools.
    ///
    /// # Arguments
    /// * `builder` - A raw pointer to an LLVMBuilderRef that needs to be stored.
    ///
    /// # Returns
    /// An option containing the newly created `BuilderTag` if successful, `None` otherwise.
    pub fn store_builder(&mut self, builder: LLVMBuilderRef) -> Option<BuilderTag> {
        let tag = BuilderTag(self.next_tag);
        self.increment_tag();        

        let c_pointer = SafeLLVMPointer::new(LLVMRef::Builder(builder))?;

        let builder_map = self.builders.get_or_insert_with(HashMap::new);
        builder_map.insert(tag, Arc::new(RwLock::new(c_pointer)));

        Some(tag)
    }

    /// Retrieves a builder by its tag from the resource pools.
    ///
    /// # Arguments
    /// * `tag` - The `BuilderTag` used to identify the builder.
    ///
    /// # Returns
    /// An option containing an `Arc<RwLock<SafeLLVMPointer>>` to the builder if found, `None` otherwise.
    pub fn get_builder(&self, tag: BuilderTag) -> Option<Arc<RwLock<SafeLLVMPointer>>> {
        self.builders.as_ref()?.get(&tag).cloned()
    }

    /// Creates a new type and stores it in the resource pools.
    ///
    /// # Arguments
    /// * `type_ref` - A raw pointer to an LLVMTypeRef that needs to be stored.
    ///
    /// # Returns
    /// An option containing the newly created `TypeTag` if successful, `None` otherwise.
    pub fn store_type(&mut self, type_ref: LLVMTypeRef) -> Option<TypeTag> {
        let tag = TypeTag(self.next_tag);
        self.increment_tag();    

        let c_pointer = SafeLLVMPointer::new(LLVMRef::Type(type_ref))?;

        let type_map = self.types.get_or_insert_with(HashMap::new);
        type_map.insert(tag, Arc::new(RwLock::new(c_pointer)));

        Some(tag)
    }

    /// Retrieves a type by its tag from the resource pools.
    ///
    /// # Arguments
    /// * `tag` - The `TypeTag` used to identify the type.
    ///
    /// # Returns
    /// An option containing an `Arc<RwLock<SafeLLVMPointer>>` to the type if found, `None` otherwise.
    pub fn get_type(&self, tag: TypeTag) -> Option<Arc<RwLock<SafeLLVMPointer>>> {
        self.types.as_ref()?.get(&tag).cloned()
    }

    /// Stores an enum definition associated with a type tag in the resource pools.
    ///
    /// # Arguments
    /// * `tag` - The `TypeTag` associated with the enum definition.
    /// * `enum_definition` - The `EnumDefinition` to be stored.
    pub fn store_enum_definition(&mut self, tag: TypeTag, enum_definition: EnumDefinition) {
        let enums_map = self.enums.get_or_insert(HashMap::new());
        enums_map.insert(tag, enum_definition);
    }

    /// Retrieves an enum definition by its associated type tag from the resource pools.
    ///
    /// # Arguments
    /// * `tag` - The `TypeTag` used to identify the enum definition.
    ///
    /// # Returns
    /// An option containing the `EnumDefinition` associated with the type tag if found, `None` otherwise.
    pub fn get_enum_definition(&self, tag: TypeTag) -> Option<EnumDefinition> {
        self.enums.as_ref()?.get(&tag).cloned()
    }

    /// Allocates a new LLVM context and stores it in the resource pool, assigning a new tag.
    ///
    /// # Returns
    /// An option containing a `ContextTag` if a new context was successfully created and stored, `None` if creation failed.
    pub fn create_context(&mut self) -> Option<ContextTag> {
        let raw_ptr: LLVMContextRef = unsafe { core::LLVMContextCreate() };

        if raw_ptr.is_null() {
            return None;
        }

        self.store_context(raw_ptr)
    }

    /// Allocates a new LLVM module in a specified context and stores it in the resource pool.
    ///
    /// # Arguments
    /// * `module_name` - The name of the module.
    /// * `context_tag` - The tag of the context where the module will be created.
    ///
    /// # Returns
    /// An option containing a `ModuleTag` if a new module was successfully created and stored, `None` if creation failed.
    pub fn create_module(&mut self, module_name: &str, context_tag: ContextTag) -> Option<ModuleTag> {
        let c_module_name: CString = CString::new(module_name).expect("Failed to create CString from module name");

        let context_arc_rwlock = self.get_context(context_tag)?;
        
        let context_rwlock = context_arc_rwlock.read().expect("Failed to lock context for reading");

        let context_ptr = context_rwlock.read(LLVMRefType::Context, |context_ref| {
            if let LLVMRef::Context(ptr) = context_ref {
                Some(*ptr)  
            } else {
                None
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
