# LLVM Intermediate Representation (IR) Generation Toolchain

## Overview
This document provides details about the `ir` module in the SafeLLVM project. The `ir` module is designed to manage and manipulate LLVM's Intermediate Representation (IR) within a safe and synchronized environment. This module handles the generation, storage, and retrieval of various LLVM IR components like modules, types, values, and basic blocks.

## Features
- **LLVM IR Management:** Generate, store, and retrieve LLVM IR components such as modules, types, values, and basic blocks.
- **Synchronized Environment:** Tagging system ensures secure management and manipulation of LLVM objects.
- **Tag-Based Object Management:** Unique identifiers for efficient retrieval and manipulation of LLVM resources.
- **High-Level Abstractions and Low-Level Control:** Provides both ease of use and direct manipulation capabilities.

## Usage

### Core 
The `core` submodule of the `ir` module is primarily intended for internal usage within the `SafeLLVM` project. It contains lower-level operations that are critical to the system's functionality but are not meant to be directly interacted with by end users without an understanding of LLVM's internal architecture. Below are examples of how some of the core functionalities are intended to be used within the `SafeLLVM` framework.

#### Creating a new Context
```rust
    let ir_gen = IRGenerator::new();
    let context_tag = ir_gen.create_context().expect("Failed to create context");
```

#### Creating and Retrieving a Module
```rust
    let module_tag = ir_gen.create_module("example_module", context_tag).expect("Failed to create module");
    let module = ir_gen.get_module(module_tag).expect("Failed to retrieve module");
```

#### Managing Values and Types
```rust
    let some_type = ...; 
    let type_tag = ir_gen.store_type(some_type).expect("Failed to store type");
    let value = ...;
    let value_tag = ir_gen.store_value(value).expect("Failed to store value");
```

#### Working with Basic Blocks and Builders
```rust
    let builder_tag = ir_gen.create_builder().expect("Failed to create builder");
    let block_tag = ir_gen.create_basic_block(builder_tag).expect("Failed to create basic block");
```

#### Enum Definitions
```rust
    let enum_def = EnumDefinition::new("ExampleEnum".to_string(), HashMap::from([
        ("Variant1", 1),
        ("Variant2", 2),
    ]));
    ir_gen.store_enum_definition(type_tag, enum_def);
```

### Block 
The `block` submodule within the `ir` module of `SafeLLVM` provides functionalities for managing basic blocks in LLVM IR. Basic blocks are sequences of instructions within LLVM that have a single entry and a single exit point, making them the building blocks of functions in LLVM IR. This submodule allows for the creation, manipulation, and querying of these basic blocks within the framework of `SafeLLVM`.

#### Creating a Basic Block
```rust
    let context_tag = ...; 
    let function_tag = ...; 
    let block_tag = ir_gen.create_basic_block(context_tag, function_tag, "entry").expect("Failed to create basic block");
```

#### Inserting a Basic Block
```rust
    let before_block_tag = ...;
    let new_block_tag = ir_gen.insert_before_basic_block(context_tag, before_block_tag, "new_block").expect("Failed to insert basic block");
```

#### Gettig the Current Active Block
```rust
    let builder_tag = ...; 
    let current_block_tag = ir_gen.get_current_block(builder_tag).expect("Failed to retrieve current block");
```

#### Creating a Conditional Branch
```rust
    let condition_tag = ...; 
    let then_block_tag = ...; 
    let else_block_tag = ...; 
    let branch_tag = ir_gen.create_cond_br(builder_tag, condition_tag, then_block_tag, else_block_tag).expect("Failed to create conditional branch");
```

#### Positioning a Builder at the End of a Block
```rust
    ir_gen.position_builder_at_end(builder_tag, block_tag).expect("Failed to position builder");
```

#### Deleting a Basic Block
```rust
    ir_gen.delete_basic_block(block_tag).expect("Failed to delete basic block");
```

#### Querying the First Instruction of a Block
```rust
    let first_instruction_tag = ir_gen.get_first_instruction(block_tag).expect("Failed to get first instruction");
```

#### Querying the Last Instruction of a Block
```rust
    let last_instruction_tag = ir_gen.get_last_instruction(block_tag).expect("Failed to get last instruction");
```

### Builder
The `builder` submodule within the `ir` module of `SafeLLVM` provides functiaonlity for constructing and manipulating LLVM's Intermediate IR. It provides an interface to handle builders, which are contexts that allow for the incremental construction of LLVM instructions. This module supports creating builders, adding functions to modules, and generating a range of arithmetic, logical, and control flow instructions.

#### Creating a Builder
```rust
    let context_tag = ...; 
    let builder_tag = ir_gen.create_builder(context_tag).expect("Failed to create builder");
```

#### Adding a Function to a Module
```rust
    let module_tag = ...;
    let function_type_tag = ...; 
    let function_tag = ir_gen.add_function_to_module(module_tag, "function", function_type_tag).expect("Failed to add function");
```

#### Building Arithmetic and Logical Operations
```rust
    let result_tag = ir_gen.build_and(builder_tag, value_tag1, value_tag2, "result").expect("Failed to build AND operation");
```

#### Building a Comparison Operation
```rust
    let result_tag = ir_gen.build_icmp_gt(builder_tag, value_tag1, value_tag2, "result").expect("Failed to build greater than comparison");
```

#### Positioning and Controlling Builders
```rust
    ir_gen.position_builder_at_end(builder_tag, block_tag).expect("Failed to position builder");
```

#### Building Shift and Bitwise Operations
```rust
    let result_tag = ir_gen.build_shl(builder_tag, value_tag, shift_amount_tag, "result").expect("Failed to build left shift");
```

#### To build a bitwise NOT operation:
```rust
    let result_tag = ir_gen.build_bitwise_not(builder_tag, value_tag, "result").expect("Failed to build bitwise NOT");
```

### Types
The `types` submodule within the `ir` module of `SafeLLVM` offers functionalities to create and manage various LLVM types. These types are essential building blocks for defining the data and behavior of the elements within LLVM IR. This submodule supports generating basic types such as integers and floats, complex types like arrays and structs, and user-defined types like enumerations.

### Values

#### Creating Basic Types
```rust
    let context_tag = ...; 
    let int_type_tag = ir_gen.int_type(context_tag, 32).expect("Failed to create 32-bit integer type");
    let float_type_tag = ir_gen.float_type(context_tag).expect("Failed to create float type");
    let boolean_type_tag = ir_gen.boolean_type(context_tag).expect("Failed to create boolean type");
```

#### Creating Complex Types
```rust
    let element_type_tag = ...; 
    let array_type_tag = ir_gen.array_type(element_type_tag, 10).expect("Failed to create array type");
    let pointer_type_tag = ir_gen.pointer_type(element_type_tag).expect("Failed to create pointer type");
```

#### Constructing Struct Types
```rust
    let context_tag = ...;
    let member_types = vec![int_type_tag, float_type_tag]; 
    let struct_type_tag = ir_gen.struct_type(context_tag, &member_types, false).expect("Failed to create struct type");
```

#### Creating Enumeration Types
```rust
    let enum_name = "Color";
    let variants = vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()];
    let enum_type_tag = ir_gen.create_enum(context_tag, 32, enum_name, &variants).expect("Failed to create enum type");
```

#### Using Types in Function Definitions
```rust
    let return_type_tag = float_type_tag; 
    let param_type_tags = vec![int_type_tag, boolean_type_tag];
    let function_type_tag = ir_gen.create_function(Some(return_type_tag), &param_type_tags, false, context_tag).expect("Failed to create function type");
```

### Variables
The `variables` submodule within the `ir` module of `SafeLLVM` offers functionalities for creating, managing, and manipulating variables within LLVM IR. It provides mechanisms for initializing variables with initial values, reassigned new values to existing variables, and retrieving variable values, all within the LLVM IR context managed by `SafeLLVM`.

#### Initializing Variables
```rust
    let builder_tag = ...; 
    let var_name = "exampleVar"; 
    let data_type_tag = ...; 
    let initial_value_tag = ...;  // Optional
    let variable_tag = ir_gen.init_var(builder_tag, var_name, data_type_tag, Some(initial_value_tag)).expect("Failed to initialize variable");
```

#### Reassigning Variables
```rust
    let builder_tag = ...; 
    let variable_alloc_tag = ...; 
    let new_value_tag = ...; 
    ir_gen.reassign_var(builder_tag, variable_alloc_tag, new_value_tag).expect("Failed to reassign variable");
```

#### Retrieving Variable Values
```rust
    let builder_tag = ...; 
    let variable_type_tag = ...; 
    let variable_alloc_tag = ...; 
    let value_tag = ir_gen.get_var(builder_tag, variable_type_tag, variable_alloc_tag).expect("Failed to retrieve variable value");
```

## FAQ

## Further Information
For further information or questions regarding the use of `ir`, feel free to contact the main contributors or raise an issue on the GitHub repository.
