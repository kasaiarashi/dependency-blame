use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents a single dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub ecosystem: EcosystemType,
    pub dependency_type: DependencyType,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EcosystemType {
    Rust,
    Node,
    Python,
    Go,
}

impl EcosystemType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EcosystemType::Rust => "Rust",
            EcosystemType::Node => "Node.js",
            EcosystemType::Python => "Python",
            EcosystemType::Go => "Go",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DependencyType {
    Direct,
    Dev,
    Optional,
    Peer,
    Build,
}

impl DependencyType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DependencyType::Direct => "Direct",
            DependencyType::Dev => "Development",
            DependencyType::Optional => "Optional",
            DependencyType::Peer => "Peer",
            DependencyType::Build => "Build",
        }
    }
}

/// Git information about when/why a dependency was added
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitInfo {
    pub commit_hash: String,
    pub author: String,
    pub date: DateTime<Utc>,
    pub message: String,
    pub file_path: PathBuf,
    pub line_number: Option<usize>,
}

/// Usage information from codebase scanning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageInfo {
    pub is_used: bool,
    pub import_locations: Vec<ImportLocation>,
    pub usage_count: usize,
}

impl Default for UsageInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl UsageInfo {
    pub fn new() -> Self {
        Self {
            is_used: false,
            import_locations: Vec::new(),
            usage_count: 0,
        }
    }

    pub fn with_locations(locations: Vec<ImportLocation>) -> Self {
        let usage_count = locations.len();
        let is_used = usage_count > 0;
        Self {
            is_used,
            import_locations: locations,
            usage_count,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportLocation {
    pub file_path: PathBuf,
    pub line_number: usize,
    pub line_content: String,
}

/// Complete analysis result for a dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyAnalysis {
    pub dependency: Dependency,
    pub git_info: Option<GitInfo>,
    pub usage_info: UsageInfo,
}

/// Query parameters
#[derive(Debug, Clone)]
pub struct DependencyQuery {
    pub dependency_name: String,
    pub repo_path: PathBuf,
    pub include_git_history: bool,
    pub scan_usage: bool,
}

impl DependencyQuery {
    pub fn new(dependency_name: String, repo_path: PathBuf) -> Self {
        Self {
            dependency_name,
            repo_path,
            include_git_history: true,
            scan_usage: true,
        }
    }

    pub fn with_options(
        dependency_name: String,
        repo_path: PathBuf,
        include_git_history: bool,
        scan_usage: bool,
    ) -> Self {
        Self {
            dependency_name,
            repo_path,
            include_git_history,
            scan_usage,
        }
    }
}
