use crate::core::dependency::{Dependency, DependencyAnalysis, DependencyQuery, UsageInfo};
use crate::core::error::{DependencyBlameError, Result};
use crate::core::git_analyzer::GitAnalyzer;
use crate::core::usage_scanner::UsageScanner;
use crate::ecosystems::registry::EcosystemRegistry;

pub struct DependencyOrchestrator {
    registry: EcosystemRegistry,
}

impl DependencyOrchestrator {
    pub fn new(registry: EcosystemRegistry) -> Self {
        Self { registry }
    }

    /// Execute a dependency query
    pub fn analyze(&self, query: DependencyQuery) -> Result<DependencyAnalysis> {
        // 1. Detect ecosystem from repo
        let ecosystem = self.registry.detect_from_directory(&query.repo_path)?;

        // 2. Get dependency file path
        let dep_file = self.registry.get_dependency_file(&query.repo_path, ecosystem)?;

        // 3. Parse dependency file to find the dependency
        let adapter = self.registry.get_adapter(ecosystem).ok_or_else(|| {
            DependencyBlameError::UnsupportedEcosystem
        })?;

        let dependency = adapter
            .parser()
            .find_dependency(&dep_file, &query.dependency_name)?
            .ok_or_else(|| {
                DependencyBlameError::DependencyNotFound(query.dependency_name.clone())
            })?;

        // 4. Get git information if requested
        let git_info = if query.include_git_history {
            match GitAnalyzer::new(&query.repo_path) {
                Ok(git_analyzer) => {
                    git_analyzer.find_dependency_introduction(&dep_file, &query.dependency_name).ok().flatten()
                }
                Err(_) => None,  // Not a git repo or error reading git
            }
        } else {
            None
        };

        // 5. Scan for usage if requested
        let usage_info = if query.scan_usage {
            let usage_scanner = UsageScanner::new(self.create_registry_copy());
            usage_scanner.scan_usage(&query.repo_path, &dependency)?
        } else {
            UsageInfo::new()
        };

        Ok(DependencyAnalysis {
            dependency,
            git_info,
            usage_info,
        })
    }

    /// List all dependencies in a project
    pub fn list_all_dependencies(&self, repo_path: &std::path::Path) -> Result<Vec<Dependency>> {
        // 1. Detect ecosystem
        let ecosystem = self.registry.detect_from_directory(repo_path)?;

        // 2. Get dependency file
        let dep_file = self.registry.get_dependency_file(repo_path, ecosystem)?;

        // 3. Parse all dependencies
        let adapter = self.registry.get_adapter(ecosystem).ok_or_else(|| {
            DependencyBlameError::UnsupportedEcosystem
        })?;

        adapter.parser().parse_dependencies(&dep_file)
    }

    // Helper to create a copy of the registry for UsageScanner
    // This is needed because UsageScanner takes ownership
    fn create_registry_copy(&self) -> EcosystemRegistry {
        crate::ecosystems::registry::create_default_registry()
    }
}
