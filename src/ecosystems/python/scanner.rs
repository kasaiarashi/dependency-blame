use crate::core::dependency::EcosystemType;
use crate::core::error::Result;
use crate::ecosystems::traits::ImportScanner;
use regex::Regex;

pub struct PythonScanner;

impl ImportScanner for PythonScanner {
    fn ecosystem_type(&self) -> EcosystemType {
        EcosystemType::Python
    }

    fn file_extensions(&self) -> Vec<&'static str> {
        vec!["py", "pyw"]
    }

    fn extract_imports(&self, content: &str) -> Result<Vec<String>> {
        let mut imports = Vec::new();

        // import module
        let import_regex = Regex::new(r"(?m)^\s*import\s+([a-zA-Z0-9_]+)")?;

        // from module import ...
        let from_import_regex = Regex::new(r"(?m)^\s*from\s+([a-zA-Z0-9_]+)\s+import")?;

        for cap in import_regex.captures_iter(content) {
            if let Some(module) = cap.get(1) {
                imports.push(module.as_str().to_string());
            }
        }

        for cap in from_import_regex.captures_iter(content) {
            if let Some(module) = cap.get(1) {
                imports.push(module.as_str().to_string());
            }
        }

        Ok(imports)
    }

    fn extract_package_name(&self, import: &str) -> String {
        // Extract the first component before . or just the whole string
        import
            .split('.')
            .next()
            .unwrap_or(import)
            .to_string()
    }

    fn normalize_package_name(&self, name: &str) -> String {
        // Python package names can use hyphens or underscores
        // Package name on PyPI might be different from import name
        // E.g., "beautifulsoup4" (PyPI) vs "bs4" (import)
        // For now, just normalize case and replace hyphens with underscores
        name.trim()
            .to_lowercase()
            .replace('-', "_")
    }
}
