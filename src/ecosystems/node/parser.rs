use crate::core::dependency::{Dependency, DependencyType, EcosystemType};
use crate::core::error::{DependencyBlameError, Result};
use crate::ecosystems::traits::DependencyParser;
use serde_json::Value;
use std::fs;
use std::path::Path;

pub struct NodeParser;

impl DependencyParser for NodeParser {
    fn ecosystem_type(&self) -> EcosystemType {
        EcosystemType::Node
    }

    fn supported_files(&self) -> Vec<&'static str> {
        vec!["package.json"]
    }

    fn parse_dependencies(&self, file_path: &Path) -> Result<Vec<Dependency>> {
        let content =
            fs::read_to_string(file_path).map_err(|e| DependencyBlameError::ParseError {
                file: file_path.display().to_string(),
                reason: e.to_string(),
            })?;

        let package_json: Value =
            serde_json::from_str(&content).map_err(|e| DependencyBlameError::ParseError {
                file: file_path.display().to_string(),
                reason: e.to_string(),
            })?;

        let mut deps = Vec::new();

        // Parse regular dependencies
        if let Some(dependencies) = package_json.get("dependencies").and_then(|v| v.as_object()) {
            for (name, value) in dependencies {
                let version = value.as_str().unwrap_or("*").to_string();
                deps.push(Dependency {
                    name: name.clone(),
                    version,
                    ecosystem: EcosystemType::Node,
                    dependency_type: DependencyType::Direct,
                });
            }
        }

        // Parse dev dependencies
        if let Some(dev_dependencies) = package_json
            .get("devDependencies")
            .and_then(|v| v.as_object())
        {
            for (name, value) in dev_dependencies {
                let version = value.as_str().unwrap_or("*").to_string();
                deps.push(Dependency {
                    name: name.clone(),
                    version,
                    ecosystem: EcosystemType::Node,
                    dependency_type: DependencyType::Dev,
                });
            }
        }

        // Parse peer dependencies
        if let Some(peer_dependencies) = package_json
            .get("peerDependencies")
            .and_then(|v| v.as_object())
        {
            for (name, value) in peer_dependencies {
                let version = value.as_str().unwrap_or("*").to_string();
                deps.push(Dependency {
                    name: name.clone(),
                    version,
                    ecosystem: EcosystemType::Node,
                    dependency_type: DependencyType::Peer,
                });
            }
        }

        // Parse optional dependencies
        if let Some(optional_dependencies) = package_json
            .get("optionalDependencies")
            .and_then(|v| v.as_object())
        {
            for (name, value) in optional_dependencies {
                let version = value.as_str().unwrap_or("*").to_string();
                deps.push(Dependency {
                    name: name.clone(),
                    version,
                    ecosystem: EcosystemType::Node,
                    dependency_type: DependencyType::Optional,
                });
            }
        }

        Ok(deps)
    }
}
