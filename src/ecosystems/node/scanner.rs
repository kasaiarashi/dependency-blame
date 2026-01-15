use crate::core::dependency::EcosystemType;
use crate::core::error::Result;
use crate::ecosystems::traits::ImportScanner;
use regex::Regex;

pub struct NodeScanner;

impl ImportScanner for NodeScanner {
    fn ecosystem_type(&self) -> EcosystemType {
        EcosystemType::Node
    }

    fn file_extensions(&self) -> Vec<&'static str> {
        vec!["js", "ts", "jsx", "tsx", "mjs", "cjs"]
    }

    fn extract_imports(&self, content: &str) -> Result<Vec<String>> {
        let mut imports = Vec::new();

        // CommonJS require: require('module') or require("module")
        let require_regex = Regex::new(r#"require\s*\(\s*['"]([^'"]+)['"]\s*\)"#)?;

        // ES6 import: import ... from 'module' or import ... from "module"
        let import_regex = Regex::new(r#"(?m)^\s*import\s+.*?from\s+['"]([^'"]+)['"]"#)?;

        // Dynamic import: import('module') or import("module")
        let dynamic_import_regex = Regex::new(r#"import\s*\(\s*['"]([^'"]+)['"]\s*\)"#)?;

        for cap in require_regex.captures_iter(content) {
            if let Some(module) = cap.get(1) {
                imports.push(module.as_str().to_string());
            }
        }

        for cap in import_regex.captures_iter(content) {
            if let Some(module) = cap.get(1) {
                imports.push(module.as_str().to_string());
            }
        }

        for cap in dynamic_import_regex.captures_iter(content) {
            if let Some(module) = cap.get(1) {
                imports.push(module.as_str().to_string());
            }
        }

        Ok(imports)
    }

    fn extract_package_name(&self, import: &str) -> String {
        // Handle scoped packages: @scope/package -> @scope/package
        // Handle subpaths: package/subpath -> package
        // Handle relative paths: ./module -> skip (not a package)

        if import.starts_with('.') || import.starts_with('/') {
            // Relative import, not a package
            return import.to_string();
        }

        if import.starts_with('@') {
            // Scoped package: @scope/package or @scope/package/subpath
            let parts: Vec<&str> = import.splitn(3, '/').collect();
            if parts.len() >= 2 {
                return format!("{}/{}", parts[0], parts[1]);
            }
        }

        // Regular package: package or package/subpath
        import
            .split('/')
            .next()
            .unwrap_or(import)
            .to_string()
    }

    fn normalize_package_name(&self, name: &str) -> String {
        // Extract the package name first (handle scoped packages and subpaths)
        let pkg_name = self.extract_package_name(name);
        pkg_name.trim().to_lowercase()
    }
}
