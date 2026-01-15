use crate::core::dependency::{Dependency, DependencyType, EcosystemType};
use crate::core::error::{DependencyBlameError, Result};
use crate::ecosystems::traits::DependencyParser;
use regex::Regex;
use std::fs;
use std::path::Path;

pub struct GoParser;

impl DependencyParser for GoParser {
    fn ecosystem_type(&self) -> EcosystemType {
        EcosystemType::Go
    }

    fn supported_files(&self) -> Vec<&'static str> {
        vec!["go.mod"]
    }

    fn parse_dependencies(&self, file_path: &Path) -> Result<Vec<Dependency>> {
        let content = fs::read_to_string(file_path).map_err(|e| {
            DependencyBlameError::ParseError {
                file: file_path.display().to_string(),
                reason: e.to_string(),
            }
        })?;

        let mut deps = Vec::new();
        let mut in_require_block = false;

        // Regex for inline require: require github.com/foo/bar v1.2.3
        let inline_require_regex = Regex::new(r"^\s*require\s+(\S+)\s+(\S+)")?;

        // Regex for require block entry: github.com/foo/bar v1.2.3
        let block_entry_regex = Regex::new(r"^\s+(\S+)\s+(\S+)")?;

        for line in content.lines() {
            let trimmed = line.trim();

            // Check for start of require block
            if trimmed.starts_with("require") && trimmed.contains('(') {
                in_require_block = true;
                continue;
            }

            // Check for end of require block
            if in_require_block && trimmed.contains(')') {
                in_require_block = false;
                continue;
            }

            // Parse inline require
            if let Some(cap) = inline_require_regex.captures(trimmed) {
                if let (Some(name), Some(version)) = (cap.get(1), cap.get(2)) {
                    deps.push(Dependency {
                        name: name.as_str().to_string(),
                        version: version.as_str().to_string(),
                        ecosystem: EcosystemType::Go,
                        dependency_type: DependencyType::Direct,
                    });
                }
            }

            // Parse require block entry
            if in_require_block {
                if let Some(cap) = block_entry_regex.captures(trimmed) {
                    if let (Some(name), Some(version)) = (cap.get(1), cap.get(2)) {
                        deps.push(Dependency {
                            name: name.as_str().to_string(),
                            version: version.as_str().to_string(),
                            ecosystem: EcosystemType::Go,
                            dependency_type: DependencyType::Direct,
                        });
                    }
                }
            }
        }

        Ok(deps)
    }
}
