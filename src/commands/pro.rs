use colored::*;
use crate::state::{elaine_dir, load_index, save_index};
use crate::project_store::create_project_if_missing;

pub fn run_pro(project_id: String) {
    if !elaine_dir().exists() {
        eprintln!("{}", "❌ Not an Elaine project. Run `eln init` first.".red());
        return;
    }

    // Ensure project exists
    create_project_if_missing(&project_id);

    // Set active project
    let mut index = load_index();
    index.active_project = Some(project_id.clone());
    save_index(&index);

    println!(
        "{}",
        format!("✔️  Active project set to '{}'", project_id)
            .bright_green()
            .bold()
    );
}
