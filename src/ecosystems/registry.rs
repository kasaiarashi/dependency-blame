use super::traits::EcosystemAdapter;
use crate::core::dependency::EcosystemType;
use crate::core::error::{DependencyBlameError, Result};
use std::collections::HashMap;
use std::path::Path;

pub struct EcosystemRegistry {
    adapters: HashMap<EcosystemType, Box<dyn EcosystemAdapter>>,
}

impl Default for EcosystemRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl EcosystemRegistry {
    pub fn new() -> Self {
        Self {
            adapters: HashMap::new(),
        }
    }

    /// Register an ecosystem adapter
    pub fn register(&mut self, adapter: Box<dyn EcosystemAdapter>) {
        self.adapters.insert(adapter.ecosystem_type(), adapter);
    }

    /// Detect ecosystem from dependency file
    pub fn detect_ecosystem(&self, file_path: &Path) -> Option<EcosystemType> {
        for adapter in self.adapters.values() {
            if adapter.parser().can_parse(file_path) {
                return Some(adapter.ecosystem_type());
            }
        }
        None
    }

    /// Detect ecosystem from a directory by looking for common dependency files
    pub fn detect_from_directory(&self, dir_path: &Path) -> Result<EcosystemType> {
        // Check for Cargo.toml
        if dir_path.join("Cargo.toml").exists() {
            return Ok(EcosystemType::Rust);
        }

        // Check for package.json
        if dir_path.join("package.json").exists() {
            return Ok(EcosystemType::Node);
        }

        // Check for Python files
        if dir_path.join("requirements.txt").exists() || dir_path.join("pyproject.toml").exists() {
            return Ok(EcosystemType::Python);
        }

        // Check for go.mod
        if dir_path.join("go.mod").exists() {
            return Ok(EcosystemType::Go);
        }

        Err(DependencyBlameError::EcosystemDetectionFailed(
            dir_path.to_path_buf(),
        ))
    }

    /// Get dependency file path for an ecosystem
    pub fn get_dependency_file(
        &self,
        dir_path: &Path,
        ecosystem: EcosystemType,
    ) -> Result<std::path::PathBuf> {
        let file_name = match ecosystem {
            EcosystemType::Rust => "Cargo.toml",
            EcosystemType::Node => "package.json",
            EcosystemType::Python => {
                // Prefer pyproject.toml, fall back to requirements.txt
                let pyproject = dir_path.join("pyproject.toml");
                if pyproject.exists() {
                    return Ok(pyproject);
                }
                "requirements.txt"
            }
            EcosystemType::Go => "go.mod",
        };

        let file_path = dir_path.join(file_name);
        if file_path.exists() {
            Ok(file_path)
        } else {
            Err(DependencyBlameError::DependencyFileNotFound(
                file_name.to_string(),
            ))
        }
    }

    /// Get adapter for ecosystem
    pub fn get_adapter(&self, ecosystem: EcosystemType) -> Option<&dyn EcosystemAdapter> {
        self.adapters.get(&ecosystem).map(|b| b.as_ref())
    }
}

/// Create a registry with all built-in ecosystems
pub fn create_default_registry() -> EcosystemRegistry {
    let mut registry = EcosystemRegistry::new();

    registry.register(Box::new(crate::ecosystems::rust::RustAdapter::new()));
    registry.register(Box::new(crate::ecosystems::node::NodeAdapter::new()));
    registry.register(Box::new(crate::ecosystems::python::PythonAdapter::new()));
    registry.register(Box::new(crate::ecosystems::go::GoAdapter::new()));

    registry
}
