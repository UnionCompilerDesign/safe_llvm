use crate::{
    memory_management::{pointer::{LLVMRef, LLVMRefType}, resource_pools::{ModuleTag, ResourcePools}},
    utils::utils_struct::Utils,
};

pub struct PackageManager {
    pub resource_pools: ResourcePools,
}

impl PackageManager {
    pub fn new() -> Self {
        Self {
            resource_pools: ResourcePools::new()
        }
    }

    pub fn write_module_to_file(&mut self, module_tag: ModuleTag, file_name: &str) -> Result<(), String> {
        let module_ref_arc = self.resource_pools.get_module(module_tag)
            .ok_or_else(|| "Module not found in resource pools".to_string())?;

        let module_ref_rwlock = module_ref_arc.read().unwrap();  

        let module_ptr = module_ref_rwlock.read(LLVMRefType::Module, |llvm_ref| {
            match llvm_ref {
                LLVMRef::Module(ptr) => Some(*ptr),
                _ => None
            }
        }).ok_or_else(|| "Failed to extract LLVM module reference".to_string())?;

        Utils::write_to_file(&module_ptr, file_name)
    }
}