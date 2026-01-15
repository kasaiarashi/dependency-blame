use crate::core::dependency::EcosystemType;
use crate::core::error::Result;
use crate::ecosystems::traits::ImportScanner;
use regex::Regex;

pub struct RustScanner;

impl ImportScanner for RustScanner {
    fn ecosystem_type(&self) -> EcosystemType {
        EcosystemType::Rust
    }

    fn file_extensions(&self) -> Vec<&'static str> {
        vec!["rs"]
    }

    fn extract_imports(&self, content: &str) -> Result<Vec<String>> {
        let mut imports = Vec::new();

        // Regex for "use" statements: use foo; use foo::bar; use foo::{bar, baz};
        let use_regex = Regex::new(r"(?m)^\s*use\s+([a-zA-Z0-9_]+)")?;

        // Regex for "extern crate" statements
        let extern_regex = Regex::new(r"(?m)^\s*extern\s+crate\s+([a-zA-Z0-9_]+)")?;

        for cap in use_regex.captures_iter(content) {
            if let Some(module) = cap.get(1) {
                imports.push(module.as_str().to_string());
            }
        }

        for cap in extern_regex.captures_iter(content) {
            if let Some(module) = cap.get(1) {
                imports.push(module.as_str().to_string());
            }
        }

        Ok(imports)
    }

    fn extract_package_name(&self, import: &str) -> String {
        // Extract the first component before :: or just the whole string
        import.split("::").next().unwrap_or(import).to_string()
    }

    fn normalize_package_name(&self, name: &str) -> String {
        // Rust crate names use hyphens in Cargo.toml but underscores in code
        // Normalize by converting to lowercase and replacing hyphens with underscores
        name.trim().to_lowercase().replace('-', "_")
    }
}
