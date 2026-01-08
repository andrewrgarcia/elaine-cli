use colored::*;
use std::io::{stdin, stdout, Write};

use crate::state::load_index;
use crate::project_store::{load_project, save_project};
use crate::reference_store::{ref_path};

pub fn run_rm(ref_id: String) {
    let index = load_index();

    let pid = match index.active_project {
        Some(p) => p,
        None => {
            eprintln!("{}", "❌ No active project set.".red());
            return;
        }
    };

    let mut project = load_project(&pid);

    if !project.refs.contains(&ref_id) {
        eprintln!(
            "{}",
            format!("❌ Reference '{}' not in project '{}'", ref_id, pid).red()
        );
        return;
    }

    project.refs.retain(|r| r != &ref_id);
    save_project(&project);

    println!(
        "{}",
        format!("🗑️  Removed '{}' from project '{}'", ref_id, pid)
            .bright_green()
    );

    // --- Check if reference is used elsewhere ----------------------------

    if !is_ref_used_elsewhere(&ref_id, &pid) {
        if confirm("Reference unused globally. Delete file too?") {
            delete_ref_file(&ref_id);
        }
    }
}


fn is_ref_used_elsewhere(ref_id: &str, current_pid: &str) -> bool {
    let projects_dir = std::path::Path::new(".elaine/projects");

    if let Ok(entries) = std::fs::read_dir(projects_dir) {
        for e in entries.flatten() {
            let path = e.path();
            if path.extension().and_then(|s| s.to_str()) != Some("yaml") {
                continue;
            }

            let contents = std::fs::read_to_string(&path).unwrap_or_default();
            if contents.contains(ref_id) && !path.to_string_lossy().contains(current_pid) {
                return true;
            }
        }
    }

    false
}

fn delete_ref_file(ref_id: &str) {
    let path = ref_path(ref_id);

    if path.exists() {
        std::fs::remove_file(&path)
            .expect("Failed to delete reference file");

        println!(
            "{}",
            format!("🧹 Deleted reference file '{}'", ref_id)
                .bright_green()
                .bold()
        );
    }
}

fn confirm(msg: &str) -> bool {
    print!("{} [y/N]: ", msg);
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}
