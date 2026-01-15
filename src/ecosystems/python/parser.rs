use crate::core::dependency::{Dependency, DependencyType, EcosystemType};
use crate::core::error::{DependencyBlameError, Result};
use crate::ecosystems::traits::DependencyParser;
use std::fs;
use std::path::Path;

pub struct PythonParser;

impl DependencyParser for PythonParser {
    fn ecosystem_type(&self) -> EcosystemType {
        EcosystemType::Python
    }

    fn supported_files(&self) -> Vec<&'static str> {
        vec!["requirements.txt", "pyproject.toml"]
    }

    fn parse_dependencies(&self, file_path: &Path) -> Result<Vec<Dependency>> {
        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        match file_name {
            "requirements.txt" => self.parse_requirements_txt(file_path),
            "pyproject.toml" => self.parse_pyproject_toml(file_path),
            _ => Err(DependencyBlameError::ParseError {
                file: file_path.display().to_string(),
                reason: "Unsupported file type".to_string(),
            }),
        }
    }
}

impl PythonParser {
    fn parse_requirements_txt(&self, file_path: &Path) -> Result<Vec<Dependency>> {
        let content = fs::read_to_string(file_path).map_err(|e| {
            DependencyBlameError::ParseError {
                file: file_path.display().to_string(),
                reason: e.to_string(),
            }
        })?;

        let mut deps = Vec::new();

        for line in content.lines() {
            let line = line.trim();

            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse dependency line (handle ==, >=, <=, ~=, etc.)
            let (name, version) = parse_requirement_line(line);

            deps.push(Dependency {
                name,
                version,
                ecosystem: EcosystemType::Python,
                dependency_type: DependencyType::Direct,
            });
        }

        Ok(deps)
    }

    fn parse_pyproject_toml(&self, file_path: &Path) -> Result<Vec<Dependency>> {
        let content = fs::read_to_string(file_path).map_err(|e| {
            DependencyBlameError::ParseError {
                file: file_path.display().to_string(),
                reason: e.to_string(),
            }
        })?;

        let pyproject: toml::Value = toml::from_str(&content).map_err(|e| {
            DependencyBlameError::ParseError {
                file: file_path.display().to_string(),
                reason: e.to_string(),
            }
        })?;

        let mut deps = Vec::new();

        // Poetry format: [tool.poetry.dependencies]
        if let Some(poetry_deps) = pyproject
            .get("tool")
            .and_then(|t| t.get("poetry"))
            .and_then(|p| p.get("dependencies"))
            .and_then(|d| d.as_table())
        {
            for (name, value) in poetry_deps {
                if name == "python" {
                    continue; // Skip Python version specifier
                }

                let version = match value {
                    toml::Value::String(s) => s.clone(),
                    toml::Value::Table(t) => {
                        t.get("version")
                            .and_then(|v| v.as_str())
                            .unwrap_or("*")
                            .to_string()
                    }
                    _ => "*".to_string(),
                };

                deps.push(Dependency {
                    name: name.clone(),
                    version,
                    ecosystem: EcosystemType::Python,
                    dependency_type: DependencyType::Direct,
                });
            }
        }

        // Poetry dev dependencies
        if let Some(dev_deps) = pyproject
            .get("tool")
            .and_then(|t| t.get("poetry"))
            .and_then(|p| p.get("dev-dependencies"))
            .and_then(|d| d.as_table())
        {
            for (name, value) in dev_deps {
                let version = match value {
                    toml::Value::String(s) => s.clone(),
                    _ => "*".to_string(),
                };

                deps.push(Dependency {
                    name: name.clone(),
                    version,
                    ecosystem: EcosystemType::Python,
                    dependency_type: DependencyType::Dev,
                });
            }
        }

        // PEP 621 format: [project.dependencies]
        if let Some(project_deps) = pyproject
            .get("project")
            .and_then(|p| p.get("dependencies"))
            .and_then(|d| d.as_array())
        {
            for dep_str in project_deps {
                if let Some(dep) = dep_str.as_str() {
                    let (name, version) = parse_requirement_line(dep);
                    deps.push(Dependency {
                        name,
                        version,
                        ecosystem: EcosystemType::Python,
                        dependency_type: DependencyType::Direct,
                    });
                }
            }
        }

        Ok(deps)
    }
}

fn parse_requirement_line(line: &str) -> (String, String) {
    // Split on common version specifiers
    for sep in &["==", ">=", "<=", "~=", ">", "<", "!="] {
        if let Some(pos) = line.find(sep) {
            let name = line[..pos].trim().to_string();
            let version = line[pos..].trim().to_string();
            return (name, version);
        }
    }

    // Handle lines with extras: package[extra]==1.0
    if let Some(bracket_pos) = line.find('[') {
        let name = line[..bracket_pos].trim().to_string();
        return (name, "*".to_string());
    }

    // No version specifier
    (line.trim().to_string(), "*".to_string())
}
