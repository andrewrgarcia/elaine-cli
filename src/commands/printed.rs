use colored::*;
use crate::state::{elaine_dir, load_index};

pub fn run_printed() {
    if !elaine_dir().exists() {
        eprintln!("{}", "❌ Not an Elaine project. Run `eln init` first.".red());
        return;
    }

    let index = load_index();

    let Some(project) = index.active_project else {
        eprintln!("{}", "❌ No active project. Use `eln pro <id>`.".red());
        return;
    };

    println!(
        "{}",
        format!("🖨️  (stub) Would print .bib for project '{}'", project)
            .bright_green()
            .bold()
    );
}
