//! # IRGeneration Toolchain for SafeLLVM
//!
//! This module provides functionalities to generate and manipulate
//! LLVM's Intermediate Representation (IR). The toolchain is designed to offer both
//! low-level direct manipulation capabilities and higher-level abstractions. It uses
//! a tag system to pair generated LLVM objects with unique identifiers.

/// This module provides functionality to build various components of the LLVM IR,
/// such as functions, basic blocks, and instructions. 
pub mod builder;

/// This module provides functionality to create values in LLVM IR. It includes interfaces 
/// to create and manipulate scalar values, composite types,and perform operations like 
/// load, store, and arithmetic computations on them.
pub mod values;

/// This module provides functionality to create types in LLVM IR.  This module provides 
/// functions to create and handle types, including integer, floating-point, and 
/// user-defined structures.
pub mod types;

/// This module provides functionality to declare and manipulate variables in LLVM IR.
/// This includes creating global and local variables, and managing their lifetimes
/// within the IR.
pub mod variables;

/// This module provides functionality to create and manage basic blocks.
pub mod block;

/// The core functionalities that are used across different modules of the IRGeneration
/// toolchain.
pub mod core;