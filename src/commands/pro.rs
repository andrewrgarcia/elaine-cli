use colored::*;
use crate::state::{load_index, save_index};

pub fn run_pro(project_id: String) {
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
