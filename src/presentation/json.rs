use crate::core::dependency::{Dependency, DependencyAnalysis};
use crate::core::error::Result;

pub fn print_analysis(analysis: &DependencyAnalysis) -> Result<()> {
    let json = serde_json::to_string_pretty(analysis)?;
    println!("{}", json);
    Ok(())
}

pub fn print_dependency_list(dependencies: &[Dependency]) -> Result<()> {
    let json = serde_json::to_string_pretty(dependencies)?;
    println!("{}", json);
    Ok(())
}
