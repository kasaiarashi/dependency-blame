use crate::core::dependency::EcosystemType;
use crate::core::error::Result;
use crate::ecosystems::traits::ImportScanner;
use regex::Regex;

pub struct GoScanner;

impl ImportScanner for GoScanner {
    fn ecosystem_type(&self) -> EcosystemType {
        EcosystemType::Go
    }

    fn file_extensions(&self) -> Vec<&'static str> {
        vec!["go"]
    }

    fn extract_imports(&self, content: &str) -> Result<Vec<String>> {
        let mut imports = Vec::new();

        // Single import: import "module"
        let single_import_regex = Regex::new(r#"(?m)^\s*import\s+"([^"]+)""#)?;

        // Multi-line import block
        let mut in_import_block = false;
        let import_entry_regex = Regex::new(r#"^\s+"([^"]+)""#)?;

        for line in content.lines() {
            let trimmed = line.trim();

            // Check for single-line import
            if let Some(cap) = single_import_regex.captures(trimmed) {
                if let Some(module) = cap.get(1) {
                    imports.push(module.as_str().to_string());
                }
                continue;
            }

            // Check for start of import block
            if trimmed.starts_with("import") && trimmed.contains('(') {
                in_import_block = true;
                continue;
            }

            // Check for end of import block
            if in_import_block && trimmed.contains(')') {
                in_import_block = false;
                continue;
            }

            // Parse import block entry
            if in_import_block {
                if let Some(cap) = import_entry_regex.captures(trimmed) {
                    if let Some(module) = cap.get(1) {
                        imports.push(module.as_str().to_string());
                    }
                }
            }
        }

        Ok(imports)
    }

    fn extract_package_name(&self, import: &str) -> String {
        // Go imports are full paths like "github.com/user/repo/package"
        // The module name in go.mod is typically "github.com/user/repo"
        // So we need to extract up to the repo level

        // For standard library, just return as-is
        if !import.contains('.') {
            return import.to_string();
        }

        // For third-party: github.com/user/repo or github.com/user/repo/v2
        let parts: Vec<&str> = import.split('/').collect();
        if parts.len() >= 3 {
            // Check if the last part is a version (v2, v3, etc.)
            let last = parts[parts.len() - 1];
            if last.starts_with('v') && last.len() <= 3 {
                // Include version in module path
                return parts[..=3.min(parts.len() - 1)].join("/");
            }
            // Standard format: domain/user/repo
            return parts[..3.min(parts.len())].join("/");
        }

        import.to_string()
    }

    fn normalize_package_name(&self, name: &str) -> String {
        name.trim().to_lowercase()
    }
}
