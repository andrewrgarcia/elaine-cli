use clap::{Parser, Subcommand};
use colored::*;
use std::path::Path;

mod state;
mod commands;
mod project;
mod project_store;
mod reference;
mod reference_store;

#[derive(Parser)]
#[command(
    name = "eln",
    version,
    about = "Elaine — CLI-first reference compiler for LaTeX users",
    long_about = "Elaine treats references as symbolic objects and compiles deterministic .bib files."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Elaine registry (.elaine/)
    Init,

    /// Show current Elaine status
    Status,

    /// Select active project
    Pro {
        project_id: String,
    },

    /// Print BibTeX for active project
    Printed,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => commands::init::run_init(),
        Commands::Status => commands::status::run_status(),
        Commands::Pro { project_id } => commands::pro::run_pro(project_id),
        Commands::Printed => commands::printed::run_printed(),
    }
}
