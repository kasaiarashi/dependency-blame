# dependency-blame

A CLI tool that analyzes project dependencies across multiple ecosystems (Rust, Node.js, Python, Go) to answer:

- **Why is this dependency here?** - Extract from git commit messages
- **Which code introduced it?** - Use git blame/log on dependency files
- **Is it still used?** - Scan codebase for import statements

## Features

- **Multi-ecosystem support**: Rust (Cargo.toml), Node.js (package.json), Python (requirements.txt/pyproject.toml), Go (go.mod)
- **Git history analysis**: Find when and why a dependency was added
- **Usage scanning**: Detect if a dependency is actually imported/used in your codebase
- **Multiple output formats**: Human-readable text, JSON, and interactive TUI (coming soon)

## Installation

```bash
cargo install --path .
```

Or build from source:

```bash
git clone <repository-url>
cd dependency-blame
cargo build --release
```

## Usage

### Analyze a specific dependency

```bash
dependency-blame analyze <dependency-name>
```

Example:
```bash
dependency-blame analyze serde
```

Options:
- `--repo <path>` - Path to the repository (default: current directory)
- `--format <text|json>` - Output format (default: text)
- `--no-git` - Skip git history analysis
- `--no-scan` - Skip usage scanning

### List all dependencies

```bash
dependency-blame list
```

Options:
- `--repo <path>` - Path to the repository (default: current directory)
- `--format <text|json>` - Output format (default: text)

### Interactive TUI (coming soon)

```bash
dependency-blame tui
```

## Example Output

### Text format:

```
============================================================
Dependency Analysis: serde v1.0.193
============================================================

Type: Direct Dependency
Ecosystem: Rust

------------------------------------------------------------
Git History:
------------------------------------------------------------
Added in: a3f8c9d
Author: John Doe <john@example.com>
Date: 2023-06-15 14:23:00 UTC
Message: Add serde for JSON serialization

------------------------------------------------------------
Usage Analysis:
------------------------------------------------------------
Status: USED (42 imports found)

Locations:
  1. src/config.rs:5
     use serde::{Deserialize, Serialize};

  2. src/models/user.rs:3
     use serde::Deserialize;

  ... and 40 more locations

============================================================
```

### JSON format:

```bash
dependency-blame analyze serde --format json
```

```json
{
  "dependency": {
    "name": "serde",
    "version": "1.0.193",
    "ecosystem": "Rust",
    "dependency_type": "Direct"
  },
  "git_info": {
    "commit_hash": "a3f8c9d...",
    "author": "John Doe <john@example.com>",
    "date": "2023-06-15T14:23:00Z",
    "message": "Add serde for JSON serialization",
    "file_path": "Cargo.toml",
    "line_number": null
  },
  "usage_info": {
    "is_used": true,
    "import_locations": [...],
    "usage_count": 42
  }
}
```

## Supported Ecosystems

- **Rust**: Parses `Cargo.toml`, scans `.rs` files for `use` statements
- **Node.js**: Parses `package.json`, scans `.js/.ts/.jsx/.tsx` files for `import`/`require`
- **Python**: Parses `requirements.txt`/`pyproject.toml`, scans `.py` files for `import`/`from` statements
- **Go**: Parses `go.mod`, scans `.go` files for `import` statements

## How It Works

1. **Ecosystem Detection**: Automatically detects the project type by looking for dependency files
2. **Dependency Parsing**: Parses the dependency file to extract all dependencies
3. **Git Analysis**: (Optional) Walks through git history to find when the dependency was introduced
4. **Usage Scanning**: (Optional) Scans all source files in parallel to detect imports

## Architecture

The tool uses a trait-based architecture for extensibility:

- **DependencyParser**: Parses ecosystem-specific dependency files
- **ImportScanner**: Scans source files for imports
- **EcosystemAdapter**: Combines parser + scanner for each ecosystem

Adding support for a new ecosystem is as simple as implementing these traits.

## Use Cases

- **Dependency cleanup**: Find unused dependencies that can be safely removed
- **Security audits**: Track when and why security-sensitive dependencies were added
- **Code archaeology**: Understand the history and rationale behind dependencies
- **Onboarding**: Help new team members understand why certain dependencies exist

## Performance

- Uses `rayon` for parallel file scanning
- Respects `.gitignore` when scanning files
- Efficient for medium-sized repos (<10k files)

## Limitations

- Git analysis requires a git repository with history
- Import scanning uses regex-based pattern matching (not full AST parsing)
- Some edge cases in import detection (dynamic imports, aliases, etc.)
- Package name normalization may not catch all cases (e.g., Python's `beautifulsoup4` → `bs4`)

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

MIT
