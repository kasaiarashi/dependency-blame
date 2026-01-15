use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "dependency-blame")]
#[command(about = "Analyze why dependencies exist in your project", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Analyze a specific dependency
    Analyze {
        /// Name of the dependency to analyze
        dependency: String,

        /// Path to the repository (defaults to current directory)
        #[arg(short, long, default_value = ".")]
        repo: PathBuf,

        /// Output format
        #[arg(short, long, value_enum, default_value = "text")]
        format: OutputFormat,

        /// Skip git history analysis
        #[arg(long)]
        no_git: bool,

        /// Skip usage scanning
        #[arg(long)]
        no_scan: bool,
    },

    /// List all dependencies in the project
    List {
        /// Path to the repository
        #[arg(short, long, default_value = ".")]
        repo: PathBuf,

        /// Output format
        #[arg(short, long, value_enum, default_value = "text")]
        format: OutputFormat,
    },

    /// Interactive TUI mode
    Tui {
        /// Path to the repository
        #[arg(short, long, default_value = ".")]
        repo: PathBuf,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    /// Human-readable text
    Text,
    /// JSON output
    Json,
}
