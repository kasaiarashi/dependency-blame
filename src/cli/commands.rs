use crate::cli::args::{Cli, Commands, OutputFormat};
use crate::core::dependency::DependencyQuery;
use crate::core::error::Result;
use crate::core::orchestrator::DependencyOrchestrator;
use crate::ecosystems::registry::create_default_registry;
use crate::presentation::{json, text, tui};

pub fn execute(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Analyze {
            dependency,
            repo,
            format,
            no_git,
            no_scan,
        } => {
            let registry = create_default_registry();
            let orchestrator = DependencyOrchestrator::new(registry);

            let query = DependencyQuery::with_options(dependency, repo, !no_git, !no_scan);

            let analysis = orchestrator.analyze(query)?;

            match format {
                OutputFormat::Text => {
                    text::print_analysis(&analysis);
                }
                OutputFormat::Json => {
                    json::print_analysis(&analysis)?;
                }
            }

            Ok(())
        }

        Commands::List { repo, format } => {
            let registry = create_default_registry();
            let orchestrator = DependencyOrchestrator::new(registry);

            let dependencies = orchestrator.list_all_dependencies(&repo)?;

            match format {
                OutputFormat::Text => {
                    text::print_dependency_list(&dependencies);
                }
                OutputFormat::Json => {
                    json::print_dependency_list(&dependencies)?;
                }
            }

            Ok(())
        }

        Commands::Tui { repo } => {
            tui::run_tui(&repo)?;
            Ok(())
        }
    }
}
