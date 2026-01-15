use crate::core::dependency::{Dependency, DependencyAnalysis};

pub fn print_analysis(analysis: &DependencyAnalysis) {
    println!("\n{}", "=".repeat(60));
    println!(
        "Dependency Analysis: {} v{}",
        analysis.dependency.name, analysis.dependency.version
    );
    println!("{}\n", "=".repeat(60));

    // Dependency info
    println!("Type: {}", analysis.dependency.dependency_type.as_str());
    println!("Ecosystem: {}", analysis.dependency.ecosystem.as_str());

    // Git history
    if let Some(git_info) = &analysis.git_info {
        println!("\n{}", "-".repeat(60));
        println!("Git History:");
        println!("{}", "-".repeat(60));
        println!("Added in: {}", &git_info.commit_hash[..8]);
        println!("Author: {}", git_info.author);
        println!("Date: {}", git_info.date.format("%Y-%m-%d %H:%M:%S UTC"));
        println!("Message:\n{}", git_info.message.trim());
    } else {
        println!("\n{}", "-".repeat(60));
        println!(
            "Git History: Not available (not a git repository or dependency history not found)"
        );
        println!("{}", "-".repeat(60));
    }

    // Usage analysis
    println!("\n{}", "-".repeat(60));
    println!("Usage Analysis:");
    println!("{}", "-".repeat(60));

    if analysis.usage_info.is_used {
        println!(
            "Status: USED ({} imports found)",
            analysis.usage_info.usage_count
        );
        println!("\nLocations:");

        let max_display = 10;
        for (i, location) in analysis
            .usage_info
            .import_locations
            .iter()
            .take(max_display)
            .enumerate()
        {
            println!(
                "  {}. {}:{}",
                i + 1,
                location.file_path.display(),
                location.line_number
            );
            println!("     {}", location.line_content);
        }

        if analysis.usage_info.usage_count > max_display {
            println!(
                "\n  ... and {} more locations",
                analysis.usage_info.usage_count - max_display
            );
        }
    } else {
        println!("Status: UNUSED (no imports found)");
        println!("\nThis dependency might be:");
        println!("  - Unused and safe to remove");
        println!("  - A transitive dependency");
        println!("  - Used in a way not detected by import scanning");
    }

    println!("\n{}\n", "=".repeat(60));
}

pub fn print_dependency_list(dependencies: &[Dependency]) {
    println!("\n{}", "=".repeat(60));
    println!("Dependencies ({} total)", dependencies.len());
    println!("{}\n", "=".repeat(60));

    let direct_deps: Vec<_> = dependencies
        .iter()
        .filter(|d| {
            matches!(
                d.dependency_type,
                crate::core::dependency::DependencyType::Direct
            )
        })
        .collect();

    let dev_deps: Vec<_> = dependencies
        .iter()
        .filter(|d| {
            matches!(
                d.dependency_type,
                crate::core::dependency::DependencyType::Dev
            )
        })
        .collect();

    let other_deps: Vec<_> = dependencies
        .iter()
        .filter(|d| {
            !matches!(
                d.dependency_type,
                crate::core::dependency::DependencyType::Direct
                    | crate::core::dependency::DependencyType::Dev
            )
        })
        .collect();

    if !direct_deps.is_empty() {
        println!("Direct Dependencies ({}):", direct_deps.len());
        println!("{}", "-".repeat(60));
        for dep in &direct_deps {
            println!("  {} ({})", dep.name, dep.version);
        }
        println!();
    }

    if !dev_deps.is_empty() {
        println!("Development Dependencies ({}):", dev_deps.len());
        println!("{}", "-".repeat(60));
        for dep in &dev_deps {
            println!("  {} ({})", dep.name, dep.version);
        }
        println!();
    }

    if !other_deps.is_empty() {
        println!("Other Dependencies ({}):", other_deps.len());
        println!("{}", "-".repeat(60));
        for dep in &other_deps {
            println!(
                "  {} ({}) - {}",
                dep.name,
                dep.version,
                dep.dependency_type.as_str()
            );
        }
        println!();
    }

    println!("{}\n", "=".repeat(60));
}
