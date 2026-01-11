use colored::*;
use std::fs;

use crate::state::{elaine_dir, load_index, save_index};
use crate::project_store::{create_project_if_missing, load_project};
use crate::utils::id::sid_short;
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
        None => list_projects(),
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

fn list_projects() {
    let projects_dir = elaine_dir().join("projects");

    let index = load_index();
    let active = index.active_project;

    let mut projects = Vec::new();

    let entries = match fs::read_dir(&projects_dir) {
        Ok(e) => e,
        Err(_) => {
            eprintln!("{}", "❌ Failed to read projects directory".red());
            return;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("yaml") {
            continue;
        }

        let id = match path.file_stem().and_then(|s| s.to_str()) {
            Some(v) => v.to_string(),
            None => continue,
        };

        let project = load_project(&id);
        projects.push(project);
    }

    if projects.is_empty() {
        println!("{}", "No projects found.".yellow());
        return;
    }

    projects.sort_by(|a, b| a.id.cmp(&b.id));

    println!("{}", "Projects:".bold());

    for project in projects {
        let marker = match &active {
            Some(a) if a == &project.id => "*",
            _ => " ",
        };

        println!(
            "{} {} ({:>2} refs) {}",
            marker,
            project.id,
            project.refs.len(),
            sid_short(&project.sid).dimmed()
        );

    }
}
