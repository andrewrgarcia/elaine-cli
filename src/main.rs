use clap::{Parser, Subcommand};

mod state;
mod commands;
mod project;
mod project_store;
mod reference;
mod reference_store;
mod bibtex;
mod utils;

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

    /// Add references (paste BibTeX via stdin)
    Add {
        /// Interactive mode
        #[arg(short = 'i', long = "interactive")]
        interactive: bool,

        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// Show current Elaine status
    Status,

    /// Select active project
    Pro {
        project_id: Option<String>,
    },


    /// Print BibTeX for active project
    Printed,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => commands::init::run_init(),
        Commands::Add { interactive, args } => commands::add::run_add(interactive, args),
        Commands::Status => commands::status::run_status(),
        Commands::Pro { project_id } => commands::pro::run_pro(project_id),
        Commands::Printed => commands::printed::run_printed(),
    }
}
