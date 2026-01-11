use colored::*;
use std::fs;

use crate::state::{elaine_dir, load_index, save_index};
use crate::project_store::create_project_if_missing;
use crate::utils::resolve_project::{resolve_project, print_project_resolve_error};

pub fn run_pro(
    project_id: Option<String>,
    delete: bool,
) {
    if !elaine_dir().exists() {
        eprintln!("{}", "❌ Not an Elaine project. Run `eln init` first.".red());
        return;
    }

    match project_id {
        Some(pid) if delete => delete_project(&pid),
        Some(pid) => switch_project(&pid),
        None => show_current_library(),
    }
}


fn delete_project(selector: &str) {
    let pid = match resolve_project(selector) {
        Ok(p) => p,
        Err(e) => {
            print_project_resolve_error(e);
            return;
        }
    };

    let path = crate::project_store::project_path(&pid);

    if !path.exists() {
        eprintln!(
            "{}",
            format!("❌ Project '{}' not found", pid).red()
        );
        return;
    }

    fs::remove_file(&path)
        .expect("❌ Failed to delete project file");

    // Clear active project if needed
    let mut index = load_index();
    if index.active_project.as_deref() == Some(&pid) {
        index.active_project = None;
        save_index(&index);
    }

    println!(
        "{}",
        format!("🗑️  Deleted project '{}' (references preserved)", pid)
            .bright_green()
            .bold()
    );
}


fn switch_project(project_id: &str) {
    create_project_if_missing(project_id);

    let mut index = load_index();
    index.active_project = Some(project_id.to_string());
    save_index(&index);

    println!(
        "{}",
        format!("✔️  Active project set to '{}'", project_id)
            .bright_green()
            .bold()
    );
}

fn show_current_library() {
    let index = load_index();

    match index.active_project {
        Some(ref pid) => {
            println!(
                "You are currently sitting in library {}",
                pid.bright_green().bold()
            );
            println!(
                "{}",
                "↳ To switch libraries: eln lib <name>"
                    .dimmed()
            );
        }
        None => {
            println!(
                "{}",
                "📚 No active library".yellow().bold()
            );
            println!(
                "{}",
                "↳ To create or switch: eln lib <name>"
                    .dimmed()
            );
        }
    }
}

fn rename_library(old: &str, new: &str) {
    let old_id = match resolve_project(old) {
        Ok(p) => p,
        Err(e) => {
            print_project_resolve_error(e);
            return;
        }
    };

    let old_path = crate::project_store::project_path(&old_id);
    let new_path = crate::project_store::project_path(new);

    if new_path.exists() {
        eprintln!(
            "{}",
            format!("❌ Library '{}' already exists", new).red()
        );
        return;
    }

    std::fs::rename(&old_path, &new_path)
        .expect("Failed to rename library file");

    // update active pointer if needed
    let mut index = load_index();
    if index.active_project.as_deref() == Some(&old_id) {
        index.active_project = Some(new.to_string());
        save_index(&index);
    }

    println!(
        "{}",
        format!("✔️  Renamed library '{}' → '{}'", old_id, new)
            .bright_green()
            .bold()
    );
}
