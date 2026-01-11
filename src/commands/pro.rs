use colored::*;
use std::fs;

use crate::state::{elaine_dir, load_index, save_index};
use crate::project_store::create_project_if_missing;
use crate::utils::resolve_project::{resolve_project, print_project_resolve_error};

pub fn run_pro(
    library_id: Option<String>,
    delete: bool,
    rename: bool,
) {
    if !elaine_dir().exists() {
        eprintln!("{}", "❌ Not an Elaine project. Run `eln init` first.".red());
        return;
    }

    match (library_id, delete, rename) {
        (Some(name), false, true) => rename_active_library(&name),
        (Some(pid), true, false) => delete_project(&pid),
        (Some(pid), false, false) => switch_project(&pid),
        (None, _, _) => show_current_library(),
        _ => {
            eprintln!(
                "{}",
                "❌ Invalid combination. Use `eln lib --rename <new_name>`".red()
            );
        }
    }
}

fn rename_active_library(new: &str) {
    let mut index = load_index();

    let old = match index.active_project.clone() {
        Some(p) => p,
        None => {
            eprintln!("{}", "❌ No active library to rename".red());
            return;
        }
    };

    let old_path = crate::project_store::project_path(&old);
    let new_path = crate::project_store::project_path(new);

    if !old_path.exists() {
        eprintln!(
            "{}",
            format!("❌ Active library '{}' not found on disk", old).red()
        );
        return;
    }

    if new_path.exists() {
        eprintln!(
            "{}",
            format!("❌ Library '{}' already exists", new).red()
        );
        return;
    }

    // 1️⃣ Load project YAML
    let contents = std::fs::read_to_string(&old_path)
        .expect("❌ Failed to read project file");

    let mut project: crate::project::Project =
        serde_yaml::from_str(&contents)
            .expect("❌ Failed to parse project YAML");

    // 2️⃣ Update canonical ID
    project.id = new.to_string();

    // 3️⃣ Write to NEW file
    let new_contents =
        serde_yaml::to_string(&project)
            .expect("❌ Failed to serialize project");

    std::fs::write(&new_path, new_contents)
        .expect("❌ Failed to write renamed project");

    // 4️⃣ Remove old file
    std::fs::remove_file(&old_path)
        .expect("❌ Failed to remove old project file");

    // 5️⃣ Update index
    index.active_project = Some(new.to_string());
    save_index(&index);

    println!(
        "{}",
        format!("✔️  Renamed active library '{}' → '{}'", old, new)
            .bright_green()
            .bold()
    );
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
        eprintln!("{}", format!("❌ Project '{}' not found", pid).red());
        return;
    }

    fs::remove_file(&path)
        .expect("❌ Failed to delete project file");

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

fn switch_project(selector: &str) {
    // 1. Try resolving existing library (by SID or name)
    let pid = match resolve_project(selector) {
        Ok(p) => p,
        Err(_) => {
            // 2. Not found → create new library with this name
            let project = create_project_if_missing(selector);
            project.id
        }
    };

    let mut index = load_index();
    index.active_project = Some(pid.clone());
    save_index(&index);

    println!(
        "{}",
        format!("✔️  Active library set to '{}'", pid)
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
            println!("{}", "↳ Rename: eln lib --rename <new_name>".dimmed());
        }
        None => {
            println!("{}", "📚 No active library".yellow().bold());
            println!("{}", "↳ Create or switch: eln lib <name>".dimmed());
        }
    }
}
