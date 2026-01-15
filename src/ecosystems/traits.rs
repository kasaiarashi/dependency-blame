use crate::core::dependency::{Dependency, EcosystemType};
use crate::core::error::Result;
use std::path::Path;

/// Trait for parsing dependency files
pub trait DependencyParser: Send + Sync {
    /// Returns the ecosystem this parser handles
    fn ecosystem_type(&self) -> EcosystemType;

    /// Returns the dependency file names this parser can handle
    fn supported_files(&self) -> Vec<&'static str>;

    /// Parse dependencies from a file
    fn parse_dependencies(&self, file_path: &Path) -> Result<Vec<Dependency>>;

    /// Check if this parser can handle the given file
    fn can_parse(&self, file_path: &Path) -> bool {
        let file_name = file_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        self.supported_files().contains(&file_name)
    }

    /// Find a specific dependency by name
    fn find_dependency(&self, file_path: &Path, dep_name: &str) -> Result<Option<Dependency>> {
        let deps = self.parse_dependencies(file_path)?;
        Ok(deps.into_iter().find(|d| d.name == dep_name))
    }
}

/// Trait for scanning codebase for imports/usage
pub trait ImportScanner: Send + Sync {
    /// Returns the ecosystem this scanner handles
    fn ecosystem_type(&self) -> EcosystemType;

    /// Returns file extensions to scan (e.g., ["rs"] for Rust)
    fn file_extensions(&self) -> Vec<&'static str>;

    /// Extract import statements from file content
    fn extract_imports(&self, content: &str) -> Result<Vec<String>>;

    /// Check if dependency is imported in the given content
    fn is_dependency_imported(&self, content: &str, dependency_name: &str) -> bool {
        self.extract_imports(content)
            .map(|imports| {
                let normalized_dep = self.normalize_package_name(dependency_name);
                imports.iter().any(|imp| {
                    let normalized_imp = self.normalize_package_name(imp);
                    normalized_imp.contains(&normalized_dep)
                        || normalized_dep.contains(&normalized_imp)
                })
            })
            .unwrap_or(false)
    }

    /// Normalize package name for matching
    fn normalize_package_name(&self, name: &str) -> String {
        // Default implementation: just trim and lowercase
        name.trim().to_lowercase()
    }

    /// Extract the base package name from an import path
    /// For example: "serde::Serialize" -> "serde", "@types/node" -> "node"
    fn extract_package_name(&self, import: &str) -> String {
        import.to_string()
    }
}

/// Combined ecosystem adapter
pub trait EcosystemAdapter: Send + Sync {
    fn parser(&self) -> &dyn DependencyParser;
    fn scanner(&self) -> &dyn ImportScanner;
    fn ecosystem_type(&self) -> EcosystemType;
}
