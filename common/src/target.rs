//! Module for handling LLVM target configurations.
//!
//! This module provides functionality to configure different processor targets for LLVM, so that
//! we can generate code for a wide variety of architectures from the same Intermediate Representation (IR).
//! Each target configurator is responsible for initializing the necessary components of LLVM
//! for a specific architecture.

extern crate llvm_sys as llvm;
use llvm::{execution_engine, target};

/// Defines the behavior for target-specific configurations.
pub trait TargetConfigurator {
    /// Configures the LLVM environment for a specific target.
    ///
    /// This method should initialize all necessary target info, target machine descriptions,
    /// MC (Machine Code) layer components, assembly parsers, and assembly printers.
    fn configure(&self);
}

/// Configurator for general target settings.
pub struct GeneralTargetConfigurator;

impl TargetConfigurator for GeneralTargetConfigurator {
    /// Implements the configuration by initializing all available targets,
    /// ensuring LLVM can target multiple architectures if required.
    fn configure(&self) {
        unsafe {
            target::LLVM_InitializeAllTargetInfos();
            target::LLVM_InitializeAllTargets();
            target::LLVM_InitializeAllTargetMCs();
            target::LLVM_InitializeAllAsmParsers();
            target::LLVM_InitializeAllAsmPrinters();
            target::LLVM_InitializeNativeTarget();
            target::LLVM_InitializeNativeAsmParser();
            target::LLVM_InitializeNativeAsmPrinter();
            execution_engine::LLVMLinkInMCJIT();
        }
    }
}

/// Configurator for ARM architecture.
pub struct ARMTargetConfigurator;

impl TargetConfigurator for ARMTargetConfigurator {
    /// Configures LLVM for ARM by calling general configuration first and then
    /// setting up ARM-specific components.
    fn configure(&self) {
        GeneralTargetConfigurator.configure();

        unsafe {
            target::LLVMInitializeARMTargetInfo();
            target::LLVMInitializeARMTarget();
            target::LLVMInitializeARMTargetMC();
            target::LLVMInitializeARMAsmParser();
            target::LLVMInitializeARMAsmPrinter();
        }
    }
}

/// Configuration for x86 architecture.
pub struct X86TargetConfigurator;

impl TargetConfigurator for X86TargetConfigurator {
    /// Configures LLVM to generate code for x86 processors by initializing
    /// the x86 specific components after the general configuration.
    fn configure(&self) {
        GeneralTargetConfigurator.configure();

        unsafe {
            target::LLVMInitializeX86TargetInfo();
            target::LLVMInitializeX86Target();
            target::LLVMInitializeX86TargetMC();
            target::LLVMInitializeX86AsmParser();
            target::LLVMInitializeX86AsmPrinter();
        }
    }
}

/// Configuration for MIPS architecture.
pub struct MIPSTargetConfigurator;

impl TargetConfigurator for MIPSTargetConfigurator {
    /// Configures LLVM to generate code for MIPS processors.
    fn configure(&self) {
        GeneralTargetConfigurator.configure();

        unsafe {
            target::LLVMInitializeMipsTargetInfo();
            target::LLVMInitializeMipsTarget();
            target::LLVMInitializeMipsTargetMC();
            target::LLVMInitializeMipsAsmParser();
            target::LLVMInitializeMipsAsmPrinter();
        }
    }
}

/// Configuration for RISC-V architecture.
pub struct RVTargetConfigurator;

impl TargetConfigurator for RVTargetConfigurator {
    /// Activates all RISC-V related LLVM initialization functions.
    fn configure(&self) {
        GeneralTargetConfigurator.configure();

        unsafe {
            target::LLVMInitializeRISCVTargetInfo();
            target::LLVMInitializeRISCVTarget();
            target::LLVMInitializeRISCVTargetMC();
            target::LLVMInitializeRISCVAsmParser();
            target::LLVMInitializeRISCVAsmPrinter();
        }
    }
}

/// Configuration for WebAssembly.
pub struct WasmTargetConfigurator;

impl TargetConfigurator for WasmTargetConfigurator {
    /// Prepares LLVM to generate WebAssembly code.
    fn configure(&self) {
        GeneralTargetConfigurator.configure();

        unsafe {
            target::LLVMInitializeWebAssemblyTargetInfo();
            target::LLVMInitializeWebAssemblyTarget();
            target::LLVMInitializeWebAssemblyTargetMC();
            target::LLVMInitializeWebAssemblyAsmParser();
            target::LLVMInitializeWebAssemblyAsmPrinter();
        }
    }
}

/// Configuration for PowerPC architecture.
pub struct PPCTargetConfigurator;

impl TargetConfigurator for PPCTargetConfigurator {
    /// Initializes all components for PowerPC code generation in LLVM.
    fn configure(&self) {
        GeneralTargetConfigurator.configure();

        unsafe {
            target::LLVMInitializePowerPCTargetInfo();
            target::LLVMInitializePowerPCTarget();
            target::LLVMInitializePowerPCTargetMC();
            target::LLVMInitializePowerPCAsmParser();
            target::LLVMInitializePowerPCAsmPrinter();
        }
    }
}

/// Configuration for SPARC architecture.
pub struct SparcTargetConfigurator;

impl TargetConfigurator for SparcTargetConfigurator {
    /// Configures LLVM to support SPARC architecture.
    fn configure(&self) {
        GeneralTargetConfigurator.configure();

        unsafe {
            target::LLVMInitializeSparcTargetInfo();
            target::LLVMInitializeSparcTarget();
            target::LLVMInitializeSparcTargetMC();
            target::LLVMInitializeSparcAsmParser();
            target::LLVMInitializeSparcAsmPrinter();
        }
    }
}

/// Configuration for SystemZ (IBM Z/Mainframe) architecture.
pub struct SystemZTargetConfigurator;

impl TargetConfigurator for SystemZTargetConfigurator {
    /// Sets up LLVM to support SystemZ architecture effectively.
    fn configure(&self) {
        GeneralTargetConfigurator.configure();

        unsafe {
            target::LLVMInitializeSystemZTargetInfo();
            target::LLVMInitializeSystemZTarget();
            target::LLVMInitializeSystemZTargetMC();
            target::LLVMInitializeSystemZAsmParser();
            target::LLVMInitializeSystemZAsmPrinter();
        }
    }
}

/// Configuration for AArch64 architecture, commonly used in modern ARM systems.
pub struct AArch64TargetConfigurator;

impl TargetConfigurator for AArch64TargetConfigurator {
    /// Initializes all necessary AArch64 components in LLVM.
    fn configure(&self) {
        GeneralTargetConfigurator.configure();

        unsafe {
            target::LLVMInitializeAArch64TargetInfo();
            target::LLVMInitializeAArch64Target();
            target::LLVMInitializeAArch64TargetMC();
            target::LLVMInitializeAArch64AsmParser();
            target::LLVMInitializeAArch64AsmPrinter();
        }
    }
}

/// Configuration for AMDGPU architecture.
pub struct AMDGPUTargetConfigurator;

impl TargetConfigurator for AMDGPUTargetConfigurator {
    /// Configures LLVM for AMDGPU targets, including assembly parsers and printers.
    fn configure(&self) {
        GeneralTargetConfigurator.configure();

        unsafe {
            target::LLVMInitializeAMDGPUTargetInfo();
            target::LLVMInitializeAMDGPUTarget();
            target::LLVMInitializeAMDGPUTargetMC();
            target::LLVMInitializeAMDGPUAsmParser();
            target::LLVMInitializeAMDGPUAsmPrinter();
        }
    }
}

/// Configuration for Berkeley Packet Filter (BPF).
pub struct BPFTargetConfigurator;

impl TargetConfigurator for BPFTargetConfigurator {
    /// Sets up necessary BPF components in LLVM for compiling to the BPF virtual machine.
    fn configure(&self) {
        GeneralTargetConfigurator.configure();

        unsafe {
            target::LLVMInitializeBPFTargetInfo();
            target::LLVMInitializeBPFTarget();
            target::LLVMInitializeBPFTargetMC();
            target::LLVMInitializeBPFAsmParser();
            target::LLVMInitializeBPFAsmPrinter();
        }
    }
}
