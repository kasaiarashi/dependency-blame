use crate::core::dependency::GitInfo;
use crate::core::error::{DependencyBlameError, Result};
use chrono::{DateTime, Utc};
use git2::{DiffOptions, Oid, Repository};
use std::path::Path;

pub struct GitAnalyzer {
    repo: Repository,
}

impl GitAnalyzer {
    pub fn new(repo_path: &Path) -> Result<Self> {
        let repo = Repository::discover(repo_path)
            .map_err(|_| DependencyBlameError::GitRepoNotFound(repo_path.to_path_buf()))?;
        Ok(Self { repo })
    }

    /// Find the commit that introduced a dependency
    pub fn find_dependency_introduction(
        &self,
        dependency_file: &Path,
        dependency_name: &str,
    ) -> Result<Option<GitInfo>> {
        // Get the file path relative to the repository root
        let repo_path = self.repo.workdir().ok_or_else(|| {
            DependencyBlameError::Other("Repository has no working directory".to_string())
        })?;

        let relative_path = dependency_file
            .strip_prefix(repo_path)
            .unwrap_or(dependency_file);

        // Walk through the commit history for this file
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;

        // Set up to walk commits in chronological order (oldest first)
        revwalk.set_sorting(git2::Sort::TIME | git2::Sort::REVERSE)?;

        let mut last_commit_without_dep: Option<Oid> = None;
        let mut first_commit_with_dep: Option<Oid> = None;

        for oid_result in revwalk {
            let oid = oid_result?;
            let commit = self.repo.find_commit(oid)?;

            // Get the tree for this commit
            let tree = commit.tree()?;

            // Try to find the dependency file in this commit
            if let Ok(entry) = tree.get_path(relative_path) {
                // File exists in this commit, check if it contains the dependency
                let object = entry.to_object(&self.repo)?;
                if let Some(blob) = object.as_blob() {
                    let content = String::from_utf8_lossy(blob.content());

                    if content.contains(dependency_name) {
                        // This commit has the dependency
                        if first_commit_with_dep.is_none() {
                            first_commit_with_dep = Some(oid);

                            // If this is the first commit we're seeing,
                            // it means the dependency was added in the first commit
                            if last_commit_without_dep.is_none() {
                                break;
                            }
                        }
                    } else {
                        // This commit doesn't have the dependency yet
                        last_commit_without_dep = Some(oid);

                        // If we already found a commit with the dep, we went too far
                        if first_commit_with_dep.is_some() {
                            break;
                        }
                    }
                }
            } else {
                // File doesn't exist in this commit yet
                last_commit_without_dep = Some(oid);
            }
        }

        // The dependency was introduced in first_commit_with_dep
        if let Some(commit_oid) = first_commit_with_dep {
            let commit = self.repo.find_commit(commit_oid)?;
            let git_info = self.extract_commit_info(&commit, relative_path)?;
            return Ok(Some(git_info));
        }

        Ok(None)
    }

    /// Get blame information for the entire dependency file
    pub fn get_dependency_history(&self, dependency_file: &Path) -> Result<Vec<GitInfo>> {
        let repo_path = self.repo.workdir().ok_or_else(|| {
            DependencyBlameError::Other("Repository has no working directory".to_string())
        })?;

        let relative_path = dependency_file
            .strip_prefix(repo_path)
            .unwrap_or(dependency_file);

        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;
        revwalk.simplify_first_parent()?;

        let mut history = Vec::new();

        for oid_result in revwalk {
            let oid = oid_result?;
            let commit = self.repo.find_commit(oid)?;

            // Check if this commit modified the dependency file
            if self.commit_modified_file(&commit, relative_path)? {
                let git_info = self.extract_commit_info(&commit, relative_path)?;
                history.push(git_info);
            }
        }

        Ok(history)
    }

    /// Extract GitInfo from a commit
    fn extract_commit_info(&self, commit: &git2::Commit, file_path: &Path) -> Result<GitInfo> {
        let author = commit.author();
        let time = commit.time();
        let timestamp = DateTime::from_timestamp(time.seconds(), 0).unwrap_or_else(Utc::now);

        Ok(GitInfo {
            commit_hash: commit.id().to_string(),
            author: format!(
                "{} <{}>",
                author.name().unwrap_or("Unknown"),
                author.email().unwrap_or("unknown@unknown.com")
            ),
            date: timestamp,
            message: commit.message().unwrap_or("No commit message").to_string(),
            file_path: file_path.to_path_buf(),
            line_number: None,
        })
    }

    /// Check if a commit modified a specific file
    fn commit_modified_file(&self, commit: &git2::Commit, file_path: &Path) -> Result<bool> {
        let tree = commit.tree()?;

        // For the first commit (no parent), check if file exists
        if commit.parent_count() == 0 {
            return Ok(tree.get_path(file_path).is_ok());
        }

        // For subsequent commits, check diff with parent
        let parent = commit.parent(0)?;
        let parent_tree = parent.tree()?;

        let mut diff_opts = DiffOptions::new();
        diff_opts.pathspec(file_path);

        let diff =
            self.repo
                .diff_tree_to_tree(Some(&parent_tree), Some(&tree), Some(&mut diff_opts))?;

        Ok(diff.deltas().len() > 0)
    }
}
