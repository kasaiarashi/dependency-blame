# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-01-15

### Added
- Initial release of dependency-blame CLI tool
- Multi-ecosystem support for analyzing dependencies:
  - Rust (Cargo.toml)
  - Node.js (package.json)
  - Python (requirements.txt, pyproject.toml)
  - Go (go.mod)
- Git history analysis to find when dependencies were introduced
- Usage scanning to detect if dependencies are imported in code
- Multiple output formats:
  - Human-readable text format with detailed sections
  - Machine-readable JSON output for scripting
- CLI commands:
  - `analyze <dependency>` - Analyze a specific dependency
  - `list` - List all dependencies in a project
  - `tui` - Interactive TUI mode (placeholder)
- CLI options:
  - `--repo <path>` - Specify repository path
  - `--format <text|json>` - Choose output format
  - `--no-git` - Skip git history analysis
  - `--no-scan` - Skip usage scanning
- Parallel file scanning with rayon for performance
- Trait-based architecture for extensibility
- Windows, Linux, and macOS binary releases
- GitHub Actions automated release workflow

### Features
- Automatic ecosystem detection from project structure
- Answers three key questions:
  - "Why is this dependency here?" - From git commit messages
  - "Which code introduced it?" - Git blame on dependency files
  - "Is it still used?" - Import scanning in source files
- Respects .gitignore when scanning files
- Comprehensive error handling with helpful messages
- Works with or without git repository
- Efficient regex-based import detection
- Support for different dependency types (direct, dev, build, etc.)

[Unreleased]: https://github.com/kasaiarashi/dependency-blame/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/kasaiarashi/dependency-blame/releases/tag/v0.1.0
