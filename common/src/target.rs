extern crate llvm_sys as llvm;

use llvm::{execution_engine, target};

pub trait TargetConfigurator {
    fn configure(&self);
}

pub struct GeneralTargetConfigurator;

impl TargetConfigurator for GeneralTargetConfigurator {
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

pub struct ARMTargetConfigurator;

impl TargetConfigurator for ARMTargetConfigurator {
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

pub struct X86TargetConfigurator;

impl TargetConfigurator for X86TargetConfigurator {
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

pub struct MIPSTargetConfigurator;

impl TargetConfigurator for MIPSTargetConfigurator {
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

pub struct RVTargetConfigurator;

impl TargetConfigurator for RVTargetConfigurator {
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

pub struct WasmTargetConfigurator;

impl TargetConfigurator for WasmTargetConfigurator {
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

pub struct PPCTargetConfigurator;

impl TargetConfigurator for PPCTargetConfigurator {
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

pub struct SparcTargetConfigurator;

impl TargetConfigurator for SparcTargetConfigurator {
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

pub struct SystemZTargetConfigurator;

impl TargetConfigurator for SystemZTargetConfigurator {
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

pub struct AArch64TargetConfigurator;

impl TargetConfigurator for AArch64TargetConfigurator {
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

pub struct AMDGPUTargetConfigurator;

impl TargetConfigurator for AMDGPUTargetConfigurator {
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

pub struct BPFTargetConfigurator;

impl TargetConfigurator for BPFTargetConfigurator {
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
