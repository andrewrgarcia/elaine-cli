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

    /// Add references (BibTeX / manual / interactive)
    Add {
        #[arg(short = 'i', long = "interactive")]
        interactive: bool,
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },

    /// Edit an existing reference
    Edit {
        ref_id: String,
    },

    /// Remove a reference from the active project
    Rm {
        ref_id: String,
    },

    /// Pin an existing reference to a project
    Pin {
        ref_id: String,
        project: Option<String>,
    },

    /// Unpin a reference from a project
    Unpin {
        ref_id: String,
        project: Option<String>,
    },

    Status {
        #[arg(short = 'v', long = "verbose")]
        verbose: bool,
    },

    /// Select or list projects
    Pro {
        project_id: Option<String>,
    },

    /// Print BibTeX for one or more projects (union)
    Printed {
        #[arg(long = "all")]
        all: bool,

        projects: Vec<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => commands::init::run_init(),
        Commands::Add { interactive, args } => commands::add::run_add(interactive, args),
        Commands::Edit { ref_id } => commands::edit::run_edit(ref_id),
        Commands::Rm { ref_id } => commands::rm::run_rm(ref_id),

        Commands::Pin { ref_id, project } =>
            commands::pin::run_pin(ref_id, project),

        Commands::Unpin { ref_id, project } =>
            commands::unpin::run_unpin(ref_id, project),

        Commands::Status { verbose } =>
            commands::status::run_status(verbose),

        Commands::Pro { project_id } =>
            commands::pro::run_pro(project_id),

        Commands::Printed { all, projects } => {
            commands::printed::run_printed(all, projects)
        }
    }
}