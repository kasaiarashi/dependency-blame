use crate::core::error::Result;
use std::path::Path;

// Basic TUI implementation - can be enhanced with ratatui later
pub fn run_tui(_repo_path: &Path) -> Result<()> {
    println!("Interactive TUI mode is not yet implemented.");
    println!("Use the 'analyze' or 'list' commands with --format text or --format json instead.");
    println!("\nExamples:");
    println!("  dependency-blame analyze serde");
    println!("  dependency-blame list");
    Ok(())
}
