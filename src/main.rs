use clap::{Parser, Subcommand};

mod state;
mod commands;
mod project;
mod project_store;
mod reference;
mod reference_store;
mod bibtex;
mod utils;
mod search;

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

    /// Attach a local document (PDF) to a reference
    Attach {
        reference: String,
        path: String,
    },

    /// Remove attachment(s) from a reference
    Detach {
        reference: String,

        /// Attachment index (1-based)
        index: Option<usize>,

        /// Remove all attachments
        #[arg(long)]
        all: bool,
    },

    /// Open attached document(s) for a reference
    Open {
        reference: String,
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
        #[arg(short = 'v', action = clap::ArgAction::Count)]
        verbose: u8,

        #[arg(long = "sort", value_parser = ["id", "title", "author", "year"])]
        sort: Option<String>,
    },


    Search {
        ref_selector: String,
    },


    /// Select or list libraries
    #[command(alias = "pro")]
    Lib {
        #[arg(long = "delete")]
        delete: bool,

        #[arg(long = "rename")]
        rename: bool,

        library_id: Option<String>,
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
        
        Commands::Attach { reference, path } =>
            commands::attach::run_attach(reference, path),

        Commands::Detach { reference, index, all } =>
            commands::detach::run_detach(reference, index, all),

        Commands::Open { reference } =>
            commands::open::run_open(reference),

        Commands::Edit { ref_id } => commands::edit::run_edit(ref_id),
        Commands::Rm { ref_id } => commands::rm::run_rm(ref_id),

        Commands::Pin { ref_id, project } =>
            commands::pin::run_pin(ref_id, project),

        Commands::Unpin { ref_id, project } =>
            commands::unpin::run_unpin(ref_id, project),

        Commands::Status { verbose, sort } =>
            commands::status::run_status(verbose, sort),

        Commands::Search { ref_selector } =>
            commands::search::run_search(ref_selector),

        Commands::Lib { library_id, delete, rename } =>
            commands::pro::run_pro(library_id, delete, rename),


        Commands::Printed { all, projects } => {
            commands::printed::run_printed(all, projects)
        }
    }
}