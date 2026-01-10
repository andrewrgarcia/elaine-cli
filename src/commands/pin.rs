use colored::*;
use crate::state::load_index;
use crate::project_store::{load_project, save_project};
use crate::reference_store::ref_exists;

pub fn run_pin(ref_id: String, project: Option<String>) {
    if !ref_exists(&ref_id) {
        eprintln!(
            "{}",
            format!("❌ Reference '{}' does not exist", ref_id).red()
        );
        return;
    }

    let index = load_index();

    let pid = match project {
        Some(p) => p,
        None => match index.active_project {
            Some(p) => p,
            None => {
                eprintln!("{}", "❌ No active project set.".red());
                return;
            }
        },
    };

    let mut proj = load_project(&pid);

    if proj.refs.contains(&ref_id) {
        println!(
            "{}",
            format!("ℹ️  Reference '{}' already pinned to '{}'", ref_id, pid)
                .yellow()
        );
        return;
    }

    proj.refs.push(ref_id.clone());
    save_project(&proj);

    println!(
        "{}",
        format!("📌 Pinned '{}' → '{}'", ref_id, pid)
            .bright_green()
            .bold()
    );
}
