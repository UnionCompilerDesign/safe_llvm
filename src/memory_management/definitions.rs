use std::collections::HashMap;

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
