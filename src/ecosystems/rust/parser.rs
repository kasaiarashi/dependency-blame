use crate::core::dependency::{Dependency, DependencyType, EcosystemType};
use crate::core::error::{DependencyBlameError, Result};
use crate::ecosystems::traits::DependencyParser;
use std::fs;
use std::path::Path;

pub struct RustParser;

impl DependencyParser for RustParser {
    fn ecosystem_type(&self) -> EcosystemType {
        EcosystemType::Rust
    }

    fn supported_files(&self) -> Vec<&'static str> {
        vec!["Cargo.toml"]
    }

    fn parse_dependencies(&self, file_path: &Path) -> Result<Vec<Dependency>> {
        let content =
            fs::read_to_string(file_path).map_err(|e| DependencyBlameError::ParseError {
                file: file_path.display().to_string(),
                reason: e.to_string(),
            })?;

        let cargo_toml: toml::Value =
            toml::from_str(&content).map_err(|e| DependencyBlameError::ParseError {
                file: file_path.display().to_string(),
                reason: e.to_string(),
            })?;

        let mut deps = Vec::new();

        // Parse regular dependencies
        if let Some(dependencies) = cargo_toml.get("dependencies").and_then(|v| v.as_table()) {
            for (name, value) in dependencies {
                let version = extract_version(value);
                deps.push(Dependency {
                    name: name.clone(),
                    version,
                    ecosystem: EcosystemType::Rust,
                    dependency_type: DependencyType::Direct,
                });
            }
        }

        // Parse dev dependencies
        if let Some(dev_dependencies) = cargo_toml
            .get("dev-dependencies")
            .and_then(|v| v.as_table())
        {
            for (name, value) in dev_dependencies {
                let version = extract_version(value);
                deps.push(Dependency {
                    name: name.clone(),
                    version,
                    ecosystem: EcosystemType::Rust,
                    dependency_type: DependencyType::Dev,
                });
            }
        }

        // Parse build dependencies
        if let Some(build_dependencies) = cargo_toml
            .get("build-dependencies")
            .and_then(|v| v.as_table())
        {
            for (name, value) in build_dependencies {
                let version = extract_version(value);
                deps.push(Dependency {
                    name: name.clone(),
                    version,
                    ecosystem: EcosystemType::Rust,
                    dependency_type: DependencyType::Build,
                });
            }
        }

        Ok(deps)
    }
}

fn extract_version(value: &toml::Value) -> String {
    match value {
        // Simple version: serde = "1.0"
        toml::Value::String(s) => s.clone(),
        // Table format: serde = { version = "1.0", features = [...] }
        toml::Value::Table(table) => table
            .get("version")
            .and_then(|v| v.as_str())
            .unwrap_or("*")
            .to_string(),
        _ => "*".to_string(),
    }
}
