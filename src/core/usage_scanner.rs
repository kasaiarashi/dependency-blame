use crate::core::dependency::{Dependency, ImportLocation, UsageInfo};
use crate::core::error::Result;
use crate::ecosystems::registry::EcosystemRegistry;
use crate::ecosystems::traits::ImportScanner;
use ignore::WalkBuilder;
use rayon::prelude::*;
use std::fs;
use std::path::Path;

pub struct UsageScanner {
    registry: EcosystemRegistry,
}

impl UsageScanner {
    pub fn new(registry: EcosystemRegistry) -> Self {
        Self { registry }
    }

    /// Scan entire codebase for dependency usage
    pub fn scan_usage(
        &self,
        repo_path: &Path,
        dependency: &Dependency,
    ) -> Result<UsageInfo> {
        let adapter = self
            .registry
            .get_adapter(dependency.ecosystem)
            .ok_or_else(|| {
                crate::core::error::DependencyBlameError::UnsupportedEcosystem
            })?;

        let scanner = adapter.scanner();
        let extensions: Vec<String> = scanner
            .file_extensions()
            .iter()
            .map(|s| s.to_string())
            .collect();

        // Build file list using ignore crate (respects .gitignore)
        let files: Vec<_> = WalkBuilder::new(repo_path)
            .hidden(false)  // Include hidden files
            .git_ignore(true)  // Respect .gitignore
            .build()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| extensions.contains(&ext.to_string()))
                    .unwrap_or(false)
            })
            .map(|entry| entry.path().to_path_buf())
            .collect();

        // Parallel scan with rayon
        let import_locations: Vec<ImportLocation> = files
            .par_iter()
            .filter_map(|file_path| {
                self.scan_file(file_path, dependency, scanner).ok()
            })
            .flatten()
            .collect();

        Ok(UsageInfo::with_locations(import_locations))
    }

    /// Scan a single file for dependency imports
    fn scan_file(
        &self,
        file_path: &Path,
        dependency: &Dependency,
        scanner: &dyn ImportScanner,
    ) -> Result<Vec<ImportLocation>> {
        let content = fs::read_to_string(file_path)?;
        let mut locations = Vec::new();

        // Check each line for imports
        for (line_num, line) in content.lines().enumerate() {
            if scanner.is_dependency_imported(line, &dependency.name) {
                locations.push(ImportLocation {
                    file_path: file_path.to_path_buf(),
                    line_number: line_num + 1,  // 1-indexed
                    line_content: line.trim().to_string(),
                });
            }
        }

        Ok(locations)
    }
}
